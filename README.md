# 🌀 Double Pendulum Simulation (Rust + Raylib)

A real-time simulation of a **double pendulum**, built using Rust and raylib.
This system is a classic example of **chaotic motion**, where tiny changes in initial conditions lead to drastically different outcomes.

---

## 🎥 Demo

👉 Add your video here (replace the link):

```
https://github.com/user-attachments/assets/3170431e-8984-4c67-a07b-23f1587ae1cc
```

---

## 🧠 What is a Double Pendulum?

A double pendulum consists of:

* Two rods connected end-to-end
* Two masses attached at the ends
* Motion governed by nonlinear dynamics

Even though it looks simple, it produces **highly unpredictable (chaotic)** motion.

---

## 📐 Equations of Motion

The system is defined by two angles:

* θ₁ → angle of first pendulum
* θ₂ → angle of second pendulum

---

### 🔹 Angular Acceleration of θ₁

θ₁'' =
(-g(2m₁ + m₂)sinθ₁ − m₂g sin(θ₁ − 2θ₂) − 2 sin(θ₁ − θ₂)m₂(θ₂'²L₂ + θ₁'²L₁ cos(θ₁ − θ₂)))
/ (L₁(2m₁ + m₂ − m₂ cos(2θ₁ − 2θ₂)))

---

### 🔹 Angular Acceleration of θ₂

θ₂'' =
(2 sin(θ₁ − θ₂)(θ₁'²L₁(m₁ + m₂) + g(m₁ + m₂)cosθ₁ + θ₂'²L₂m₂ cos(θ₁ − θ₂)))
/ (L₂(2m₁ + m₂ − m₂ cos(2θ₁ − 2θ₂)))

---

## ⚙️ Variables

| Symbol     | Meaning              |
| ---------- | -------------------- |
| θ₁, θ₂     | Angles (radians)     |
| θ₁', θ₂'   | Angular velocity     |
| θ₁'', θ₂'' | Angular acceleration |
| L₁, L₂     | Length of rods       |
| m₁, m₂     | Masses               |
| g          | Gravity constant     |

---

## 🖥️ Implementation Details

* Language: Rust 🦀
* Graphics: raylib
* Math: Trigonometric physics equations
* Integration: Euler method (simple update loop)

---

## 🔁 Update Logic

Each frame:

```
θ₁' += θ₁'' * dt
θ₂' += θ₂'' * dt

θ₁  += θ₁' * dt
θ₂  += θ₂' * dt
```

---

## ⚠️ Notes

* The system is **chaotic** → small changes = huge differences
* Large time steps (`dt`) can cause instability
* For better accuracy, use **RK4 integration**

---

## 🚀 Features

* Real-time rendering
* Physics-based motion
* Modular drawing functions
* Easy to extend (trails, damping, UI)

---

## 🔮 Future Improvements

* RK4 integration
* Energy visualization
* Trail rendering
* Interactive controls (change mass, length, etc.)

---

## 📦 How to Run

```bash
cargo run
```

---

## 🤝 Contributing

Feel free to fork and experiment — chaos welcomes everyone 😈

---

## 📜 License

MIT License
