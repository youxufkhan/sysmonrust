# Project Requirements Document (PRD): Lightweight Linux System Monitor

## 1. Project Overview
**Name:** sysmonrust
**Objective:** Build a highly performant, lightweight desktop system monitor for Ubuntu and Linux Mint. The app will visualize real-time system resource usage and sensor data using a minimalist, aesthetically pleasing graph-based UI.

## 2. Technology Stack
* **Framework:** Tauri v2
* **Backend:** Rust
    * *Crates:* `sysinfo` (CPU, RAM, Disk, Network), `sensors` (temps), `nvml-wrapper` (NVIDIA GPUs)
* **Frontend:** Svelte + TypeScript
* **Charting Library:** uPlot (60fps, lightweight)
* **Styling:** Tailwind CSS + shadcn-svelte

## 3. Core Features & Requirements

### 3.1 Data Metrics
* **CPU:** Overall %, per-core %, clock speeds, temperature
* **Memory:** RAM (total, used, available), Swap
* **Network:** RX/TX speeds, total data transferred
* **Disk:** I/O speeds, capacity (root + home)
* **GPU:** Usage %, VRAM, temperature (NVIDIA + AMD + Intel with fallback)

### 3.2 UI/UX
* Dark mode by default
* Real-time scrolling line charts (60s history)
* htop-style bar gauges for CPU cores
* System tray integration
* Target: <30MB RAM usage
