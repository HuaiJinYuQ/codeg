//! CRUD for user-registered ACP agents (`custom_agent` table).
//!
//! Rows here are the persistent form of [`CustomAgentDef`]; the process-global
//! launch registry (`crate::acp::custom_registry`) is rebuilt from them by
//! [`hydrate_registry`] at startup and after every mutation.

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet,
    QueryFilter, QueryOrder, Set,
};

use crate::acp::custom_registry::{
    self, CustomAgentDef, CustomAgentSource, CustomAgentSpec, CustomDistributionKind,
    FALLBACK_VERSION,
};
use crate::db::entities::custom_agent;
use crate::db::error::DbError;
use crate::models::agent::is_valid_custom_agent_id;

/// Convert a stored row into a launch definition. Returns `None` when the row
/// is unusable (corrupt `spec_json`, unknown distribution kind) — the caller
/// skips it rather than failing the whole hydrate.
pub fn def_from_model(model: &custom_agent::Model) -> Option<CustomAgentDef> {
    let spec: CustomAgentSpec = serde_json::from_str(&model.spec_json).ok()?;
    let distribution_kind = CustomDistributionKind::parse(&model.distribution_kind)?;
    Some(CustomAgentDef {
        registry_id: model.registry_id.clone(),
        name: model.name.clone(),
        description: model.description.clone(),
        version: model.version.clone(),
        distribution_kind,
        spec,
        icon_url: model.icon_url.clone(),
        skills_shared_store: model.skills_shared_store,
        skills_dir: model.skills_dir.clone(),
        source: CustomAgentSource::parse(&model.source),
        version_probe: model
            .version_probe
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string),
        supports_mcp: model.supports_mcp,
    })
}

pub async fn list(conn: &DatabaseConnection) -> Result<Vec<custom_agent::Model>, DbError> {
    Ok(custom_agent::Entity::find()
        .order_by_asc(custom_agent::Column::RegistryId)
        .all(conn)
        .await?)
}

pub async fn list_defs(conn: &DatabaseConnection) -> Result<Vec<CustomAgentDef>, DbError> {
    let rows = list(conn).await?;
    Ok(rows
        .iter()
        .filter_map(|row| {
            let def = def_from_model(row);
            if def.is_none() {
                tracing::warn!(
                    "[custom-agent] skipping unreadable row {:?} (bad spec_json or distribution_kind)",
                    row.registry_id
                );
            }
            def
        })
        .collect())
}

pub async fn get(
    conn: &DatabaseConnection,
    registry_id: &str,
) -> Result<Option<custom_agent::Model>, DbError> {
    Ok(custom_agent::Entity::find()
        .filter(custom_agent::Column::RegistryId.eq(registry_id))
        .one(conn)
        .await?)
}

/// Insert a new definition, or update the existing row with the same
/// `registry_id`. Validation happens up front so a row that could never launch
/// is never persisted.
pub async fn upsert(conn: &DatabaseConnection, def: &CustomAgentDef) -> Result<(), DbError> {
    if !is_valid_custom_agent_id(&def.registry_id) {
        return Err(DbError::Migration(format!(
            "invalid custom agent id: {}",
            def.registry_id
        )));
    }
    // Reject anything that cannot be turned into launch metadata — the same
    // check `hydrate` would apply, but here it can still be reported to the
    // user, and before the row lands rather than after. Uses `validate` rather
    // than `build_meta` so a rejected save leaks nothing; the metadata itself is
    // built (and kept) by the `hydrate` that follows a successful save.
    custom_registry::validate(def).map_err(|e| DbError::Migration(e.to_string()))?;

    let spec_json =
        serde_json::to_string(&def.spec).map_err(|e| DbError::Migration(e.to_string()))?;
    let version = if def.version.trim().is_empty() {
        FALLBACK_VERSION.to_string()
    } else {
        def.version.trim().to_string()
    };
    let now = Utc::now();

    match get(conn, &def.registry_id).await? {
        Some(existing) => {
            let mut active = existing.into_active_model();
            active.name = Set(def.name.trim().to_string());
            active.description = Set(def.description.trim().to_string());
            active.version = Set(version);
            active.distribution_kind = Set(def.distribution_kind.as_str().to_string());
            active.spec_json = Set(spec_json);
            active.icon_url = Set(def.icon_url.clone());
            active.skills_shared_store = Set(def.skills_shared_store);
            active.skills_dir = Set(def.skills_dir.clone());
            active.source = Set(def.source.as_str().to_string());
            active.version_probe = Set(def.version_probe.clone());
            active.supports_mcp = Set(def.supports_mcp);
            active.updated_at = Set(now);
            active.update(conn).await?;
        }
        None => {
            custom_agent::ActiveModel {
                id: NotSet,
                registry_id: Set(def.registry_id.clone()),
                name: Set(def.name.trim().to_string()),
                description: Set(def.description.trim().to_string()),
                version: Set(version),
                distribution_kind: Set(def.distribution_kind.as_str().to_string()),
                spec_json: Set(spec_json),
                icon_url: Set(def.icon_url.clone()),
                skills_shared_store: Set(def.skills_shared_store),
                skills_dir: Set(def.skills_dir.clone()),
                source: Set(def.source.as_str().to_string()),
                version_probe: Set(def.version_probe.clone()),
                supports_mcp: Set(def.supports_mcp),
                created_at: Set(now),
                updated_at: Set(now),
            }
            .insert(conn)
            .await?;
        }
    }
    Ok(())
}

/// Remove a definition. Conversations that reference it keep their
/// `custom:<id>` agent type and their recorded transcripts — they simply become
/// unlaunchable, which is what an uninstalled agent should look like.
pub async fn delete(conn: &DatabaseConnection, registry_id: &str) -> Result<bool, DbError> {
    let result = custom_agent::Entity::delete_many()
        .filter(custom_agent::Column::RegistryId.eq(registry_id))
        .exec(conn)
        .await?;
    Ok(result.rows_affected > 0)
}

/// Rebuild the process-global launch registry from the database. Call at
/// startup and after every mutation.
pub async fn hydrate_registry(conn: &DatabaseConnection) -> Result<(), DbError> {
    let defs = list_defs(conn).await?;
    let errors = custom_registry::hydrate(&defs);
    for (id, err) in errors {
        tracing::warn!("[custom-agent] {id} is registered but not launchable: {err}");
    }
    Ok(())
}

/// 预置 sahaa 智能体。
///
/// 若 `sahaa` 尚未在数据库中注册，则写入一条默认定义：
/// - `registry_id`: `"sahaa"`
/// - `name`: `"Sahaa"`
/// - `distribution_kind`: `npx`（通过 PATH 上的 `sahaa` 命令启动）
///
/// 此函数为幂等操作：若记录已存在则跳过，不覆盖用户修改。
pub async fn seed_sahaa_agent(conn: &DatabaseConnection) -> Result<(), DbError> {
    use crate::acp::custom_registry::{CustomAgentSpec, NpxSpec};
    use std::collections::BTreeMap;

    const REGISTRY_ID: &str = "sahaa";

    // 检查是否已存在精确匹配（小写 sahaa）
    if get(conn, REGISTRY_ID).await?.is_some() {
        return Ok(());
    }

    // 兼容旧数据：若存在大写 ID 的历史记录（如 'Sahaa'），直接更新修正，而非重复插入
    let existing_rows = list(conn).await?;
    let stale = existing_rows
        .into_iter()
        .find(|r| r.registry_id.to_lowercase() == REGISTRY_ID);

    if let Some(row) = stale {
        // 有旧记录但 ID 大小写不对，直接修正
        use sea_orm::IntoActiveModel;
        let mut active = row.into_active_model();
        active.registry_id = Set(REGISTRY_ID.to_string());
        active.name = Set("Sahaa".to_string());
        active.description = Set("Sahaa AI coding agent".to_string());
        active.version = Set("custom".to_string());
        active.distribution_kind = Set("npx".to_string());
        active.spec_json = Set(
            r#"{"npx":{"package":"sahaa","args":[],"env":{},"cmd":"sahaa"}}"#.to_string(),
        );
        active.source = Set("manual".to_string());
        active.supports_mcp = Set(true);
        active.updated_at = Set(Utc::now());
        active.update(conn).await?;
        hydrate_registry(conn).await?;
        tracing::info!("[custom-agent] sahaa agent ID corrected (was uppercase)");
        return Ok(());
    }

    // sahaa 通过 PATH 直接启动，使用 npx 分发类型（package = cmd = "sahaa"）
    // validate 对 npx 只要求 package 非空，符合要求
    let def = CustomAgentDef {
        registry_id: REGISTRY_ID.to_string(),
        name: "Sahaa".to_string(),
        description: "Sahaa AI coding agent".to_string(),
        version: "custom".to_string(),
        distribution_kind: CustomDistributionKind::Npx,
        spec: CustomAgentSpec {
            npx: Some(NpxSpec {
                package: "sahaa".to_string(),
                cmd: Some("sahaa".to_string()),
                args: vec![],
                env: BTreeMap::new(),
                node_required: None,
            }),
            uvx: None,
            binary: BTreeMap::new(),
        },
        icon_url: Some("data:image/png;base64,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".to_string()),
        skills_shared_store: false,
        skills_dir: None,
        source: CustomAgentSource::Manual,
        version_probe: None,
        supports_mcp: true,
    };

    // 使用 upsert 写入（内部有 validate，失败则记录警告但不中断启动）
    if let Err(e) = upsert(conn, &def).await {
        tracing::warn!("[custom-agent] seed sahaa: upsert failed: {e}");
        return Ok(());
    }

    // 写入后立即重新水化注册表，使 sahaa 立刻可用
    hydrate_registry(conn).await?;

    // 确保 agent_setting 中 sahaa 排第一且 enabled
    // 先清理大写 ID 的旧记录（历史遗留）
    crate::db::service::agent_setting_service::remove_stale_sahaa(&conn).await?;
    // 将 sahaa 置于 sort_order=0，其余 agent 顺延
    crate::db::service::agent_setting_service::pin_sahaa_as_default(&conn).await?;

    tracing::info!("[custom-agent] sahaa agent seeded successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acp::custom_registry::{BinaryPlatformSpec, NpxSpec};
    use crate::db::test_helpers::fresh_in_memory_db;
    use std::collections::BTreeMap;

    fn npx_def(id: &str) -> CustomAgentDef {
        CustomAgentDef {
            registry_id: id.to_string(),
            name: "Qwen Code".into(),
            description: "desc".into(),
            version: "0.21.0".into(),
            distribution_kind: CustomDistributionKind::Npx,
            spec: CustomAgentSpec {
                npx: Some(NpxSpec {
                    package: "@qwen-code/qwen-code@0.21.0".into(),
                    args: vec!["--acp".into()],
                    cmd: Some("qwen".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            icon_url: Some("https://example.com/i.svg".into()),
            skills_shared_store: false,
            skills_dir: None,
            source: Default::default(),
            version_probe: None,
            supports_mcp: true,
        }
    }

    #[tokio::test]
    async fn upsert_inserts_then_updates_in_place() {
        let db = fresh_in_memory_db().await;
        upsert(&db.conn, &npx_def("qwen-code")).await.unwrap();
        assert_eq!(list(&db.conn).await.unwrap().len(), 1);

        let mut edited = npx_def("qwen-code");
        edited.name = "Qwen (edited)".into();
        edited.version = "0.22.0".into();
        upsert(&db.conn, &edited).await.unwrap();

        let rows = list(&db.conn).await.unwrap();
        assert_eq!(rows.len(), 1, "same registry_id must update, not duplicate");
        assert_eq!(rows[0].name, "Qwen (edited)");
        assert_eq!(rows[0].version, "0.22.0");
        assert!(rows[0].updated_at >= rows[0].created_at);
    }

    #[tokio::test]
    async fn upsert_rejects_definitions_that_could_never_launch() {
        let db = fresh_in_memory_db().await;

        let mut bad_id = npx_def("qwen-code");
        bad_id.registry_id = "../escape".into();
        assert!(upsert(&db.conn, &bad_id).await.is_err());

        // Declares the npx channel but supplies only a binary spec.
        let mut wrong_channel = npx_def("mismatch");
        wrong_channel.spec.npx = None;
        wrong_channel.spec.binary.insert(
            "linux-x86_64".into(),
            BinaryPlatformSpec {
                archive: "https://example.com/a.tar.gz".into(),
                cmd: "./a".into(),
                ..Default::default()
            },
        );
        assert!(upsert(&db.conn, &wrong_channel).await.is_err());

        assert!(list(&db.conn).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn blank_version_falls_back_rather_than_persisting_empty() {
        let db = fresh_in_memory_db().await;
        let mut def = npx_def("no-version");
        def.version = "   ".into();
        upsert(&db.conn, &def).await.unwrap();
        assert_eq!(get(&db.conn, "no-version").await.unwrap().unwrap().version, FALLBACK_VERSION);
    }

    #[tokio::test]
    async fn the_mcp_opt_out_persists_and_is_editable_in_place() {
        let db = fresh_in_memory_db().await;
        let mut def = npx_def("mcp-row");
        def.supports_mcp = false;
        upsert(&db.conn, &def).await.unwrap();
        let row = get(&db.conn, "mcp-row").await.unwrap().unwrap();
        assert!(!row.supports_mcp);
        assert!(!def_from_model(&row).expect("readable").supports_mcp);

        // The update arm has to write it too, or turning MCP back on would
        // appear to save and silently do nothing.
        def.supports_mcp = true;
        upsert(&db.conn, &def).await.unwrap();
        assert!(get(&db.conn, "mcp-row").await.unwrap().unwrap().supports_mcp);
    }

    #[tokio::test]
    async fn delete_reports_whether_a_row_was_removed() {
        let db = fresh_in_memory_db().await;
        upsert(&db.conn, &npx_def("gone")).await.unwrap();
        assert!(delete(&db.conn, "gone").await.unwrap());
        assert!(!delete(&db.conn, "gone").await.unwrap());
        assert!(get(&db.conn, "gone").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn unreadable_rows_are_skipped_not_fatal() {
        let db = fresh_in_memory_db().await;
        upsert(&db.conn, &npx_def("good-one")).await.unwrap();
        // Corrupt a row the way a hand-edited DB or a future schema would.
        let broken = custom_agent::ActiveModel {
            id: NotSet,
            registry_id: Set("broken-one".into()),
            name: Set("Broken".into()),
            description: Set(String::new()),
            version: Set("1".into()),
            distribution_kind: Set("carrier-pigeon".into()),
            spec_json: Set("{not json".into()),
            icon_url: Set(None),
            skills_shared_store: Set(false),
            skills_dir: Set(None),
            source: Set("registry".into()),
            version_probe: Set(None),
            supports_mcp: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        broken.insert(&db.conn).await.unwrap();

        let defs = list_defs(&db.conn).await.unwrap();
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].registry_id, "good-one");
    }

    #[tokio::test]
    // The guard is held across awaits on purpose: it is a test-only mutex that
    // no production code ever takes, and `#[tokio::test]` runs this future on a
    // single-threaded runtime, so it cannot block a worker or deadlock. Holding
    // it for the whole test is the point — `hydrate_registry` replaces a
    // process-global map, so a concurrent test must not interleave.
    #[allow(clippy::await_holding_lock)]
    async fn hydrate_registry_publishes_rows_to_the_launch_registry() {
        let _guard = custom_registry::hydrate_test_guard();
        let db = fresh_in_memory_db().await;
        upsert(&db.conn, &npx_def("hydrate-svc-agent")).await.unwrap();
        hydrate_registry(&db.conn).await.unwrap();
        assert!(custom_registry::is_registered("hydrate-svc-agent"));
        let meta = custom_registry::get("hydrate-svc-agent").unwrap();
        assert_eq!(meta.name, "Qwen Code");

        delete(&db.conn, "hydrate-svc-agent").await.unwrap();
        hydrate_registry(&db.conn).await.unwrap();
        assert!(!custom_registry::is_registered("hydrate-svc-agent"));
    }

    #[tokio::test]
    async fn def_round_trips_through_the_row() {
        let db = fresh_in_memory_db().await;
        let mut def = npx_def("round-trip");
        def.skills_shared_store = true;
        def.skills_dir = Some("/opt/agent/skills".into());
        def.source = CustomAgentSource::Manual;
        def.version_probe = Some("qwen --version".into());
        upsert(&db.conn, &def).await.unwrap();
        let row = get(&db.conn, "round-trip").await.unwrap().unwrap();
        let back = def_from_model(&row).expect("readable");
        assert_eq!(back.registry_id, def.registry_id);
        assert_eq!(back.distribution_kind, def.distribution_kind);
        assert_eq!(back.icon_url, def.icon_url);
        assert!(back.skills_shared_store);
        assert_eq!(back.skills_dir.as_deref(), Some("/opt/agent/skills"));
        assert_eq!(back.source, CustomAgentSource::Manual);
        assert_eq!(back.version_probe.as_deref(), Some("qwen --version"));
        assert!(back.supports_mcp, "the default survives the row");
        let npx = back.spec.npx.expect("npx channel survives");
        assert_eq!(npx.package, "@qwen-code/qwen-code@0.21.0");
        assert_eq!(npx.cmd.as_deref(), Some("qwen"));
        assert_eq!(npx.args, vec!["--acp".to_string()]);
        let _ = BTreeMap::<String, String>::new();
    }
}
