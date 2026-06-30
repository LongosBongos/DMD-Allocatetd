<p align="center">
  <img src="https://raw.githubusercontent.com/LongosBongos/DMD-Allocatetd/main/DMD.png" alt="Die Mark Digital Logo" width="200"/>
</p>

<p align="center">
  <a href="https://longosbongos.github.io/Investor_App_DMD/">
    <img src="https://img.shields.io/badge/🚀_Launch_Investor_App-Gold?style=for-the-badge&logo=solana&logoColor=white&color=d4af37" alt="Launch DMD App"/>
  </a>
</p>

<h1 align="center">💰 Die Mark Digital (DMD)</h1>

<p align="center">
  <a href="https://solana.com">
    <img src="https://img.shields.io/badge/Blockchain-Solana-14f195?logo=solana&logoColor=white" alt="Solana"/>
  </a>
  <a href="https://spl.solana.com/token">
    <img src="https://img.shields.io/badge/Token-SPL-yellow" alt="SPL Token"/>
  </a>
  <img src="https://img.shields.io/badge/Status-Public_Reference-brightgreen" alt="Public Reference"/>
  <img src="https://img.shields.io/badge/Verified_Build-Confirmed-success" alt="Verified Build"/>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License"/>
  </a>
  <a href="https://github.com/LongosBongos/DMD-Allocatetd/blob/main/AUDIT_CHECKLIST.md">
    <img src="https://img.shields.io/badge/Audit-Checklist-success" alt="Audit Checklist"/>
  </a>
</p>

---

## 💰 Die Mark Digital

Die Digitale Deutsche Mark (DMD) ist ein Solana-basiertes Token-Projekt mit Vault-, Treasury- und Regel-Logik.

Ziel ist eine strukturierte Infrastruktur mit nachvollziehbarer On-Chain-Architektur, öffentlicher IDL, klarer Rollen-Trennung und langfristig ausgerichteter Treasury-Mechanik.

Dieses Repository dient als öffentliche Referenz- und Verifikationsschicht für DMD. Es stellt public-safe Informationen, Projekt-Referenzen, Policy-, Security- und Audit-Checklist-Dokumentation bereit.

---

## 🔑 Projekt-Infos

| Feld                      | Wert                                           |
| ------------------------- | ---------------------------------------------- |
| Program ID                | `EDY4bp4fXWkAJpJhXUMZLL7fjpDhpKZQFPpygzsTMzro` |
| Mint                      | `3rCZT3Xw6jvU4JWatQPsivS8fQ7gV7GjUfJnbTk9Ssn5` |
| Vault PDA                 | `AfbZG6WHh462YduimCUmAvVi3jSjGfkaQCyEnYPeXwPF` |
| Treasury Wallet           | `9fAjEDdFjmGwwxh5fyUhDsbyg8RwE7TR12Y25iD4FCoS` |
| Protocol Owner            | `GsnjzePaFi2fq4wBYDuRYSfXiMQ1NsFmAYVdhvKUWoXm` |
| Admin / Upgrade Authority | `EGPTLNcdpG4vpfo3thjWJ5FEiPk3n88ppR1dtHTKejbP` |
| Supply                    | `150,000,000 DMD`                              |
| Decimals                  | `9`                                            |
| Token Standard            | `SPL Token`                                    |

---

## ✅ Verifizierter Build-Status

Das DMD-Programm wurde erfolgreich gegen den öffentlichen Repository-Stand reproduzierbar gebaut, mit dem On-Chain-Programm abgeglichen und anschließend als Verifikationsnachweis auf Solana hochgeladen.

| Feld                      | Wert                                                                                       |
| ------------------------- | ------------------------------------------------------------------------------------------ |
| Verification Result       | `Program hash matches`                                                                     |
| Verified Repo             | `https://github.com/LongosBongos/DMD-Allocatetd`                                           |
| Verified Commit           | `515dfea53f38562fada57a2c75fbedfc531c0811`                                                 |
| Program Hash              | `30068ed51330a15c696b92a5ad08b9f655646b98d46687d75e40e5a98277a2a6`                         |
| Verification Upload Tx    | `2C6fsEGFfKeG986k57MnjpyFDGKvNXBbtAvcU1NuoU1hk47iS8G7dN8Cpw3a6nPfejWAPpz1zZmc2wEpPQycw32b` |
| Upgrade Authority Removed | `No`                                                                                       |

### 🔍 Reproducible Verification

This program was verified using the official Solana reproducible build process.

* Build environment: `Solana v2.3.0`
* Docker build used for deterministic compilation
* Binary hash matches exactly with the on-chain program
* Verification data was uploaded on-chain after hash confirmation

> Note: A verified build confirms that the published source and the on-chain program hash match. It does not replace an independent external security audit.

---

## ⚙️ Aktueller On-Chain-Status

| Feld            | Status                         |
| --------------- | ------------------------------ |
| Public Sale     | `active`                       |
| Dynamic Pricing | `true`                         |
| Sell Live       | `false`                        |
| IDL             | `public reference available`   |
| Anchor Source   | `public-safe source published` |
| Verified Build  | `confirmed`                    |

---

## 🧭 Sale / Access Logic Clarification

DMD is currently documented with public sale mode as the relevant active access structure.

The contract contains legacy self-whitelist logic from the earlier presale/access phase. This logic remains documented for transparency, but it should not be confused with the current public sale access model.

Current public sale buy limits:

* Minimum buy amount: `0.1 SOL`
* Maximum buy amount: `100 SOL`
* Public sale access: `active` according to the current documented project state
* Sell status: depends on the active on-chain system state and treasury configuration

Important distinction:

* The minimum buy amount defines how much SOL a user can contribute in a buy transaction.
* The legacy self-whitelist balance check defines whether a wallet can pass the earlier presale/access condition.
* These are separate mechanisms.

DMD documents this distinction openly so users can understand the difference between legacy presale access logic, public sale mode, buy limits and active system state.

---

## 🌐 Offizielle Links

| Bereich           | Link                                                                   |
| ----------------- | ---------------------------------------------------------------------- |
| Investor App      | https://longosbongos.github.io/Investor_App_DMD/                       |
| Telegram          | https://t.me/diemarkDigitaloffiziell                                   |
| X / Twitter       | https://x.com/DieMarkDigital                                           |
| Policy            | https://longosbongos.github.io/DMD-Allocatetd/policy.html              |
| Security.txt      | https://longosbongos.github.io/DMD-Allocatetd/.well-known/security.txt |
| Public Repository | https://github.com/LongosBongos/DMD-Allocatetd                         |

---

## 📄 Öffentliche Referenzen

| Referenz              | Datei                        |
| --------------------- | ---------------------------- |
| IDL                   | `target/idl/dmd_anchor.json` |
| Audit Checklist       | `AUDIT_CHECKLIST.md`         |
| Security Policy       | `security.txt`               |
| Token Metadata        | `metadata.dmd2.json`         |
| Public Reference Page | `index.html`                 |
| Project Policy        | `policy.html`                |

---

## 🧩 Öffentliche Source-Struktur

Dieses Repository enthält:

* öffentliche DMD-IDL
* public-safe Anchor-Source zur Verifikation
* Policy-, Audit- und Security-Referenzen
* statische Dateien für die öffentliche Projektpräsenz
* öffentliche Projekt- und Systemreferenzen

Dieses Repository enthält nicht:

* Wallet-Dateien
* `.env`-Dateien
* lokale Admin-/Deploy-Tools
* private Treasury-/Authority-Skripte
* sensible Schlüssel oder Seed-Daten

---

## 🛡️ Transparenz- und Sicherheits-Hinweis

DMD stellt öffentliche Referenzen, eine verifizierte Build-Dokumentation und eine Audit-Checklist bereit.

Die Audit-Checklist ist eine öffentliche Projekt- und Sicherheits-Checkliste. Sie ist nicht als vollständiger Ersatz für ein unabhängiges externes Security-Audit zu verstehen.

Nutzer sollten die öffentlichen Referenzen, den Investor-App-Status und den On-Chain-Systemstatus eigenständig prüfen, bevor sie mit DMD interagieren.

---

## ⚠️ Disclaimer

### Deutsch

Die Mark Digital (DMD) ist ein blockchainbasiertes Projekt mit eigener Treasury- und Regel-Logik.

Dieses Repository stellt öffentliche Projektinformationen, Referenzen und technische Dokumentation bereit. Es stellt keine Rechts-, Steuer- oder Finanzberatung dar.

DMD ist kein gesetzliches Zahlungsmittel und wird in diesem Repository nicht als Aktie, Wertpapier oder garantiertes Anlageprodukt dargestellt. Eine rechtliche Einordnung kann je nach Rechtsordnung und Einzelfall unterschiedlich ausfallen.

Jede Nutzung erfolgt eigenverantwortlich und auf eigenes Risiko.

Es bestehen keine Ansprüche auf Rückzahlung, Erstattung oder garantierte Rendite.

Technische Änderungen, Marktbewegungen, Drittfehler, Smart-Contract-Risiken und On-Chain-Risiken können jederzeit Einfluss auf Nutzung, Verfügbarkeit und Wert haben.

### English

Die Mark Digital (DMD) is a blockchain-based project with treasury and rule-based mechanics.

This repository provides public project information, references and technical documentation. It does not constitute legal, tax or financial advice.

DMD is not legal tender and is not presented in this repository as a stock, security or guaranteed investment product. Legal classification may vary depending on jurisdiction and individual circumstances.

Any use is entirely at the user's own responsibility and risk.

There is no claim for repayment, refund or guaranteed return.

Technical changes, market movements, third-party failures, smart contract risks and on-chain risks may affect usage, availability and value at any time.

---

## 📬 Kontakt

Email: [diemarkdigital@gmail.com](mailto:diemarkdigital@gmail.com)
- [DMD Handbook / Handbuch](./DMD_Handbuch_Die_Mark_Digital_DE_EN_FR.pdf)

## Official Documents

- [DMD Whitepaper / Structure Memo DE/EN v1.0](./DMD_Whitepaper_Structure_Memo_v1_0.pdf)
---

<p align="center">
  © 2026 <b>Die Mark Digital (DMD)</b><br/>
  Built on Solana • Public IDL • Public-safe Anchor Source • Verified Build • MIT License
</p>

