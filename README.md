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
│     Signal Processing Layer  │
│ Noise Generator + MA Filter  │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│             BPCS             │
│ Proportional Level Control   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│              SIS             │
│ Safety Interlock Protection  │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│  Feed Water Tank Process     │
│ (SDV + Pump + LT Simulation) │
└──────────────────────────────┘
```

---

# ⚙️ Features

✔ Real-time GUI visualization  
✔ Industrial P&ID background  
✔ Moving Average sensor filtering  
✔ Random noise simulation  
✔ Proportional BPCS control  
✔ SIS interlock protection  
✔ Overflow prevention  
✔ Dry-run pump protection  
✔ Immediate-mode rendering system

---

# 🛠️ Technologies Used

| Component | Technology |
|---|---|
| Programming Language | Rust |
| GUI Framework | eframe / egui |
| Image Loader | egui_extras |
| Random Generator | rand |
| Image Decoder | image crate |

---

# 📁 Project Directory Structure

```text
Simulation_Feed_Water_Tank/
│
├── src/
│   └── main.rs
│
├── assets/
│   └── background.png
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

# 🛠️ Dependencies

```toml
[dependencies]
eframe      = "0.28.1"
egui_extras = { version = "0.28.1", features = ["all_loaders"] }
image       = { version = "0.24", features = ["png"] }
rand        = "0.8"
```

---

# ▶️ Running the Application

## 1️⃣ Clone Repository

```bash
git clone https://github.com/sintiiaa08-cyber/TUGAS-ALPROG-KELOMPOK-22.git
```

---

## 2️⃣ Enter Project Directory

```bash
cd TUGAS-ALPROG-KELOMPOK-22
```

---

## 3️⃣ Run Application

```bash
cargo run
```

---

# 🎮 System Operation

## A. Normal Operation

- Operator adjusts tank level using slider
- Sensor signal is processed through:
  - Random noise generator
  - Moving Average filter
- BPCS maintains level near setpoint

Default Setpoint:

```text
SP = 50%
```

---

## B. SIS Protection Logic

### Overflow Protection

If:

```text
PV ≥ 90%
```

Then:

- Inlet SDV closes automatically

---

### Dry Run Protection

If:

```text
PV ≤ 10%
```

Then:

- Outlet SDV closes
- Pump shuts down automatically

---

# 📊 Signal Processing

Moving Average Equation:

```text
PV_filtered = (1/N) Σ(x_raw + η)
```

Where:

- N = 5 samples
- x_raw = operator input
- η = random noise

---

# 🧠 Control Algorithm

BPCS proportional controller:

```text
CV_BPCS = Kp × (SP − PV_filtered)
```

Where:

- Kp = 1.5
- SP = 50%

---

# 🏁 Project Status

✔ Fully operational  
✔ Real-time GUI simulation working  
✔ Stable SIS interlock implementation  
✔ Ready for academic presentation  
✔ Suitable for industrial automation learning

---

# 🎓 Application Context

This project is suitable for:

- Industrial automation learning
- Process control simulation
- Instrumentation engineering education
- SIS/BPCS architecture studies
- Rust GUI development practice

---

# 📚 References

1. eframe & egui Documentation  
2. egui_extras Documentation  
3. Rust rand Crate Documentation  
4. IEC 61511 Functional Safety Standard  
5. Bela G. Liptak — Instrument Engineers’ Handbook
