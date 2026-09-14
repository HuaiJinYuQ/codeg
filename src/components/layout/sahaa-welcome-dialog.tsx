"use client"

import { useState, useEffect } from "react"
import { DownloadIcon, BookOpenIcon } from "lucide-react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { BrowserLink } from "@/components/ui/browser-link"
import { Label } from "@/components/ui/label"

/** localStorage key：用户选择"今后都不显示"后写入此 key */
const DISMISS_KEY = "sahaa-welcome-dialog-dismissed"

const DOWNLOAD_URL =
  "https://workdrive.zoho.in/folder/1etqy0708390c2fcf46e6b88ed15d705ed8d1?layout=list"
const DOCS_URL =
  "https://sahaa-docs-60066246735.development.catalystserverless.in/app/getting-started/installing-sahaa"

/**
 * 首次打开工作台时弹出的 Sahaa 安装提示框。
 *
 * - 仅在用户从未勾选"今后都不显示"时展示
 * - 提供下载链接与官网指南链接
 * - 底部"今后都不显示"复选框勾选并关闭后永久压制
 */
export function SahaaWelcomeDialog() {
  const [open, setOpen] = useState(false)
  const [neverShow, setNeverShow] = useState(false)

  // 客户端挂载后检查是否已被永久压制
  useEffect(() => {
    if (typeof window === "undefined") return
    const dismissed = localStorage.getItem(DISMISS_KEY)
    if (!dismissed) {
      setOpen(true)
    }
  }, [])

  function handleClose() {
    if (neverShow) {
      localStorage.setItem(DISMISS_KEY, "1")
    }
    setOpen(false)
  }

  return (
    <Dialog open={open} onOpenChange={(v) => !v && handleClose()}>
      <DialogContent
        className="max-w-lg"
        showCloseButton
        onPointerDownOutside={(e) => e.preventDefault()}
      >
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-xl">
            <img
              src="/zoho-logo/sahaa-yali-logo.png"
              alt="Sahaa"
              className="size-8 rounded-md object-contain"
            />
            安装 Sahaa CLI
          </DialogTitle>
          <DialogDescription className="text-sm leading-relaxed">
            Codez 使用 Sahaa 作为内置 AI 编码智能体。请先安装 Sahaa CLI
            才能开始使用。
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4">
          <div className="rounded-2xl bg-muted/50 p-4 text-sm leading-relaxed space-y-2">
            <p className="font-medium text-foreground">安装步骤：</p>
            <ol className="list-decimal list-inside space-y-1.5 text-muted-foreground">
              <li>点击下方“前往下载”打开 Zoho WorkDrive</li>
              <li>下载对应平台的 Sahaa CLI 安装包</li>
              <li>按照官方指南完成安装，确保{" "}
                <code className="rounded bg-muted px-1 py-0.5 text-xs font-mono">
                  sahaa
                </code>{" "}
                命令可在终端中运行
              </li>
            </ol>
          </div>

          <div className="flex flex-col gap-2 sm:flex-row">
            <Button className="flex-1 gap-2" asChild>
              <BrowserLink href={DOWNLOAD_URL}>
                <DownloadIcon className="size-4" />
                前往下载
              </BrowserLink>
            </Button>
            <Button variant="outline" className="flex-1 gap-2" asChild>
              <BrowserLink href={DOCS_URL}>
                <BookOpenIcon className="size-4" />
                官方安装指南
              </BrowserLink>
            </Button>
          </div>
        </div>

        <DialogFooter className="flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div className="flex items-center gap-2">
            <Checkbox
              id="sahaa-never-show"
              checked={neverShow}
              onCheckedChange={(checked) => setNeverShow(checked === true)}
            />
            <Label
              htmlFor="sahaa-never-show"
              className="cursor-pointer text-sm text-muted-foreground select-none"
            >
              今后都不显示
            </Label>
          </div>

          <Button
            variant="secondary"
            onClick={handleClose}
            className="sm:ml-auto"
          >
            我知道了
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
