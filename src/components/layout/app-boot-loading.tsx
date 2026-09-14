"use client"

import { Loader2 } from "lucide-react"

export function AppBootLoading() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-background text-foreground">
      <div className="flex items-center gap-3 px-2 py-1">
        <img src="/icon.svg" alt="Codez" className="h-5 w-5" />
        <span className="text-sm font-medium tracking-tight">codez</span>
        <Loader2 className="h-4 w-4 animate-spin text-primary" />
      </div>
    </div>
  )
}
