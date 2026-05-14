# 💻 TUGAS-ALPROG-KELOMPOK-22

Simulation Feed Water Tank Monitoring & Control System Berbasis GUI dengan Integrasi BPCS dan SIS

---

## 👩‍💻 Prepared by

- Fauzi Abdul Rozaq (2042241017)
- Sintia Ompusunggu (2042241113)

Department of Instrumentation Engineering  
Institut Teknologi Sepuluh Nopember (ITS)

---

# 📘 Project Overview

This project presents the development of a graphical simulation system for monitoring and controlling a Feed Water Tank (FWT) using the Rust programming language.

The system integrates:

- Basic Process Control System (BPCS)
- Safety Instrumented System (SIS)
- Real-time GUI visualization
- Moving Average signal filtering
- Industrial safety interlock logic

The application is designed to simulate industrial boiler feed water monitoring systems using modern GUI architecture based on `eframe` and `egui`.

---

# 🧩 System Architecture

```text
┌──────────────────────────────┐
│        Operator GUI          │
│  (Rust + eframe + egui)      │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│     Signal Processing Layer   │
│ Noise Generator + MA Filter   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│              BPCS             │
│  Proportional Level Control   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│               SIS             │
│ Safety Interlock Protection   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│   Feed Water Tank Process     │
│ (SDV + Pump + LT Simulation)  │
└──────────────────────────────┘
