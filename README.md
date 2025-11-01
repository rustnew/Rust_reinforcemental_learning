<img width="1427" height="1053" alt="Capture d’écran du 2025-10-31 23-11-27" src="https://github.com/user-attachments/assets/01eccfd6-b1ac-49f0-91df-4e93d6641530" />

# 🚀 RocketRL — Simulation & Reinforcement Learning for Reusable Rocket Landing (Rust + Bevy)

### 🧠 Contrôler, stabiliser et faire atterrir une fusée réutilisable avec l’apprentissage par renforcement et Rust  (RL)

---

## 📘 Sommaire

1. [Introduction](#-introduction)
2. [Objectifs du projet](#-objectifs-du-projet)
3. [Concept et architecture](#-concept-et-architecture)
4. [Fonctionnalités principales](#-fonctionnalités-principales)
5. [Structure du projet](#-structure-du-projet)
6. [Environnement de simulation](#-environnement-de-simulation)
7. [Apprentissage par renforcement](#-apprentissage-par-renforcement)
8. [Technologies utilisées](#-technologies-utilisées)
9. [Installation & Exécution](#-installation--exécution)
10. [Feuille de route](#-feuille-de-route)
11. [Applications réelles & vision](#-applications-réelles--vision)
12. [Licence](#-licence)

---

## 🪐 Introduction

**RocketRL** est un projet de recherche et développement open-source codé en **Rust**, conçu pour **simuler et entraîner un agent d’apprentissage par renforcement** à effectuer **un atterrissage autonome de fusée réutilisable**.

L’objectif est de **reproduire les conditions physiques réalistes** d’un lancement spatial (descente, poussée, gravité, vent, carburant limité) afin de concevoir un **algorithme capable de piloter la fusée de manière stable, précise et économe**.

Ce projet explore les mêmes défis rencontrés par des entreprises comme **SpaceX**, **Blue Origin**, ou **RocketLab**, mais dans un environnement libre et optimisé pour la recherche.

---

## 🎯 Objectifs du projet

### Objectif principal :

> Développer une **simulation physique complète** et un **agent RL performant** capable de **faire atterrir une fusée réutilisable verticalement**, avec **une précision maximale et une consommation minimale de carburant.**

### Objectifs secondaires :

* Créer un **simulateur physique temps réel** en Rust (avec Bevy + Rapier).
* Implémenter des **algorithmes RL continus** (DDPG, PPO, SAC) directement ou via interfaçage Python.
* Étudier la **robustesse** de la politique apprise (vent, masse, erreurs capteurs).
* Fournir un **outil open-source pédagogique et industriel** pour tester des stratégies de contrôle avancées.

---

## 🧩 Concept et architecture

Le projet repose sur l’architecture **ECS (Entity Component System)** de **Bevy**, combinée à un moteur de physique (Rapier) et un module RL dédié.

```
┌──────────────────────────────────────────┐
│                RocketRL                  │
├──────────────────────────────────────────┤
│  🧱 ECS Core (Bevy)                      │
│     ├─ Entities: Rocket, LandingPad      │
│     ├─ Components: Position, Velocity…   │
│     └─ Systems: Thrust, Gravity, Render  │
│                                          │
│  🧮 Physics Engine (Rapier)              │
│     └─ Simulation des forces, collisions │
│                                          │
│  🧠 RL Module                            │
│     ├─ State Encoder                     │
│     ├─ Reward Function                   │
│     ├─ PPO / DDPG Agent                  │
│     └─ Trainer / Evaluator               │
└──────────────────────────────────────────┘
```

---

## 🛠️ Fonctionnalités principales

✅ **Simulation physique réaliste**

* Gravité, poussée variable, rotation, masse, résistance atmosphérique
* Gestion du carburant et de la stabilité

✅ **Apprentissage par renforcement**

* Agent RL apprenant à stabiliser et poser la fusée
* Récompenses dynamiques basées sur la précision, la consommation et la stabilité

✅ **Environnement paramétrable**

* Gravité ajustable (Terre, Lune, Mars)
* Conditions météo (vent, turbulence)
* Hauteur initiale, masse variable

✅ **Visualisation 2D / 3D temps réel**

* Vue top-down (2D)
* Vue libre 3D (caméra orbitale)
* Affichage des forces, trajectoires, et états

✅ **Performances maximales**

* 100% Rust, sans dépendances lourdes en Python
* Simulation multi-threadée optimisée

---

## 🧱 Structure du projet

```
RocketRL/
├── src/
│   ├── main.rs              # Point d’entrée Bevy
│   ├── physics.rs           # Gravité, forces, moteur Rapier
│   ├── rocket.rs            # Entité fusée et logique moteur
│   ├── environment.rs       # Paramètres de simulation
│   ├── rl/
│   │   ├── agent.rs         # Implémentation de l’agent RL
│   │   ├── reward.rs        # Calcul des récompenses
│   │   └── training.rs      # Boucle d’entraînement
│   └── utils.rs             # Fonctions utilitaires
│
├── assets/                  # Modèles, textures, sons
├── Cargo.toml               # Dépendances Rust
└── README.md                # Ce fichier 😎
```

---

## 🌍 Environnement de simulation

### 🔸 États (state vector)

```rust
[state = {
   altitude,
   velocity_y,
   velocity_x,
   angle,
   angular_velocity,
   fuel_remaining,
   wind_force,
}]
```

### 🔸 Actions (continuous)

```rust
[action = {
   thrust_power ∈ [0, 1],
   gimbal_angle ∈ [-15°, +15°]
}]
```

### 🔸 Récompense

```rust
reward =
   +1000  if landing_successful
   -distance_to_pad * 2.0
   -abs(angle) * 0.5
   -fuel_used * 0.1
   -1000 if crash
```

---

## 🤖 Apprentissage par renforcement

### Algorithmes prévus :

* **Phase 1 :** DDPG (Deep Deterministic Policy Gradient)
* **Phase 2 :** PPO (Proximal Policy Optimization)
* **Phase 3 :** SAC (Soft Actor-Critic)

### Boucle d’apprentissage (simplifiée) :

1. Initialiser les poids du réseau.
2. Exécuter un épisode de simulation.
3. Calculer les récompenses et transitions `(s, a, r, s')`.
4. Mettre à jour la politique.
5. Répéter jusqu’à convergence.

---

## 🧰 Technologies utilisées

| Composant        | Outil / Crate Rust                        | Rôle                             |
| ---------------- | ----------------------------------------- | -------------------------------- |
| 🎮 Moteur de jeu | `bevy`                                    | Rendu graphique + ECS            |
| ⚙️ Physique      | `bevy_rapier2d` / `bevy_rapier3d`         | Gestion des forces et collisions |
| 🧠 RL Core       | `burn`, `linfa`, ou implémentation maison | Réseaux de neurones + PPO/DDPG   |
| 📈 Visualisation | `bevy_prototype_lyon`, `egui`             | Graphiques, HUD, statistiques    |
| 🧪 Tests         | `criterion`, `assert_approx_eq`           | Benchmark et validation          |

---

## 💻 Installation & Exécution

### Prérequis

* Rust 1.80+
* Cargo
* GPU compatible Vulkan / OpenGL

### Installation

```bash
git clone https://github.com/tonpseudo/RocketRL.git
cd RocketRL
cargo run
```

### Contrôles manuels (mode test)

| Touche | Action                 |
| ------ | ---------------------- |
| ↑      | Augmenter poussée      |
| ← / →  | Gimbal gauche / droite |
| Space  | Couper moteur          |
| R      | Redémarrer simulation  |

---

## 📅 Feuille de route

| Étape | Description                              | Statut      |
| ----- | ---------------------------------------- | ----------- |
| 1️⃣   | Simulation physique 2D fonctionnelle     | ✅           |
| 2️⃣   | Visualisation Bevy + contrôles manuels   | ✅           |
| 3️⃣   | Ajout de la logique de récompense RL     | 🔄 En cours |
| 4️⃣   | Implémentation DDPG                      | 🔜          |
| 5️⃣   | Extension en 3D (Rapier3D)               | ⏳           |
| 6️⃣   | Optimisation multi-thread RL             | ⏳           |
| 7️⃣   | Démonstrateur embarqué / microcontrôleur | 🚀 Planifié |

---

## 🌌 Applications réelles & vision

* **Industrie spatiale :** test et validation de systèmes d’atterrissage autonomes
* **Robotique mobile :** transfert des politiques RL vers drones / robots équilibrés
* **IA embarquée :** preuve de concept d’IA de contrôle léger en Rust
* **Recherche open-source :** environnement de référence pour l’étude du RL physique

💡 Vision à long terme :

> Créer le premier simulateur *entièrement Rust* de contrôle de fusée réutilisable,
> combinant performance, sûreté et apprentissage automatique.

---

## 🧾 Licence

Ce projet est sous licence **MIT** — libre d’utilisation, de modification et de distribution, à condition de conserver la mention de l’auteur.

---

## 👨‍🚀 Auteur

**Martial Wato**
🚀 Ingénieur / Développeur Rust passionné par l’aérospatial et l’IA
📫 Contact : *[[rustspeakmastery@gmail.com](rustspeakmastery@gmail.com)]*
🌍 Projet open-source made in Rust.

