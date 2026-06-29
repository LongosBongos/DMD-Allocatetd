# 🔍 Die Mark Digital (DMD) – Smart Contract Audit Checklist

**Program ID:** `EDY4bp4fXWkAJpJhXUMZLL7fjpDhpKZQFPpygzsTMzro`
**Token Standard:** SPL Token
**Network:** Solana Mainnet
**Date:** 2025-10-05
**Audited by:** DMD Founder (Pre-Audit Self-Check)

---

## Important Notice

This document is a public pre-audit checklist and founder self-check for transparency purposes.

It documents reviewed contract areas, known mechanics, public references and current project assumptions. It is not a replacement for an independent external security audit.

Users, reviewers and contributors should verify the public source, IDL, on-chain state and project references independently before interacting with DMD.

---

## 🧩 1. Overflow & Arithmetic Checks

| Test                        | Beschreibung                                                                                                                                                                                       | Ergebnis |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `u64` / `BN` Berechnungen   | Rechenoperationen für SOL-Beiträge, Rewards, Penalties und Limits werden mit Integer-basierten Datentypen verarbeitet. Overflow-Checks sind im Release-Build aktiviert (`overflow-checks = true`). | ✅ Passed |
| Reward & Penalty Skalierung | Rewards und Penalties nutzen prozentbasierte Multiplikation mit Division durch konstante Basiswerte.                                                                                               | ✅ Passed |
| Treasury-Splits             | Splits werden über feste Ganzzahl-Multiplikation durchgeführt; keine Float-Berechnung.                                                                                                             | ✅ Passed |
| Contribution Limits         | Mindest- und Maximalbeiträge werden getrennt geprüft.                                                                                                                                              | ✅ Passed |

---

## 🧾 2. Signer & Authority Validation

| Test                 | Beschreibung                                                                                                                                                                          | Ergebnis |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| Founder-Signaturen   | Founder-Operationen wie `initialize`, `set_manual_price`, `toggle_public_sale` und `whitelist_add` verlangen eine Signer-Prüfung auf die definierte Founder-/Protocol-Owner-Adresse.  | ✅ Passed |
| Treasury Authority   | Treasury-bezogene Operationen verlangen die jeweils definierte Treasury-/Authority-Struktur gemäß Programmlogik.                                                                      | ✅ Passed |
| Buyer Auth           | Käufertransaktionen wie `buy_dmd`, `claim_reward`, `sell_dmd_v2`, `swap_exact_sol_for_dmd` oder vergleichbare Nutzerfunktionen prüfen die Nutzer-/Buyer-Signatur gemäß Programmlogik. | ✅ Passed |
| PDA-Ownership        | Vault- und BuyerState-PDAs validieren Seeds gegen die Program ID.                                                                                                                     | ✅ Passed |
| Authority Separation | Founder, Treasury, Vault und BuyerState sind als getrennte Rollen/Strukturen dokumentiert.                                                                                            | ✅ Passed |

---

## 💰 3. Treasury & Split Logic

| Test                | Beschreibung                                                                                                                                              | Ergebnis     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| Presale-Split       | Presale-/Legacy-Logik enthält definierte Split-Mechaniken für Founder und Treasury.                                                                       | ✅ Documented |
| Buy/Sell Fees       | Buy- und Sell-Fee-Mechaniken sind im Contract und in der Projektdokumentation als Regelbestandteil dokumentiert.                                          | ✅ Documented |
| Reward Pool         | Rewards basieren auf definierter Treasury-/Vault-Logik. Es wird kein unbegrenzter Mint-Inflation-Mechanismus als Reward-Quelle dokumentiert.              | ✅ Passed     |
| Withdraws           | Keine offene externe `withdraw()`-Funktion für beliebige Nutzer dokumentiert. Treasury-Verwendung bleibt an definierte Rollen und Programmlogik gebunden. | ✅ Passed     |
| Treasury Dependency | Sell- und Reward-Funktionen können vom aktiven Systemzustand, Treasury-Zustand und Vault-Bestand abhängen.                                                | ✅ Documented |

---

## 🏦 4. Vault / BuyerState Integrity

| Test              | Beschreibung                                                                                                           | Ergebnis |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------- | -------- |
| Vault PDA         | Vault-PDA ist öffentlich dokumentiert: `AfbZG6WHh462YduimCUmAvVi3jSjGfkaQCyEnYPeXwPF`.                                 | ✅ Passed |
| BuyerState-PDA    | BuyerState-Struktur nutzt definierte Seeds zur nutzerbezogenen Statusverwaltung.                                       | ✅ Passed |
| Rent-Exemption    | Accounts wie `Vault` und `BuyerState` werden gemäß Solana-/Anchor-Accountmodell erstellt.                              | ✅ Passed |
| Data Layout       | Account-Strukturen sind Anchor-kompatibel aufgebaut und enthalten definierte Felder für Status, Beträge und Zeitlogik. | ✅ Passed |
| Public References | Relevante öffentliche Referenzen werden im README und in der Public Reference Page dokumentiert.                       | ✅ Passed |

---

## 🪙 5. Token Logic & Supply Control

| Test                 | Beschreibung                                                                                           | Ergebnis     |
| -------------------- | ------------------------------------------------------------------------------------------------------ | ------------ |
| Mint Supply          | Gesamtmenge ist öffentlich dokumentiert: `150,000,000 DMD`.                                            | ✅ Passed     |
| Decimals             | Decimals sind öffentlich dokumentiert: `9`.                                                            | ✅ Passed     |
| Token Standard       | DMD nutzt SPL-Token-Logik auf Solana.                                                                  | ✅ Passed     |
| Vault-ATA            | Vault-/Token-Bestände werden über definierte Vault-/ATA-Strukturen verwaltet.                          | ✅ Passed     |
| Mint / Freeze Status | Mint- und Freeze-Authority-Status sind in den öffentlichen Projekt- und Metadata-Referenzen zu prüfen. | ✅ Documented |

---

## ⚙️ 6. Reward / Penalty Mechanism

| Test                 | Beschreibung                                                                                  | Ergebnis |
| -------------------- | --------------------------------------------------------------------------------------------- | -------- |
| Reward Logic         | Reward-Funktionen basieren auf definierter Vault-/Treasury- und Zeitlogik.                    | ✅ Passed |
| Hold Duration        | Haltefrist-Logik ist als Schutz- und Regelmechanik dokumentiert.                              | ✅ Passed |
| Claim Interval       | Reward-/Claim-Intervalle sind als zeitbasierte Logik dokumentiert.                            | ✅ Passed |
| Penalty Logic        | Penalty-Mechaniken sind als Schutz gegen kurzfristige oder regelwidrige Nutzung dokumentiert. | ✅ Passed |
| No Guaranteed Return | Rewards werden nicht als garantierte Rendite oder garantierter Anspruch dargestellt.          | ✅ Passed |

---

## 🧰 7. Whitelist & Public Sale

| Test                    | Beschreibung                                                                                                | Ergebnis     |
| ----------------------- | ----------------------------------------------------------------------------------------------------------- | ------------ |
| Legacy Self-Whitelist   | `auto_whitelist_self` gehört zur früheren Presale-/Access-Struktur und bleibt zur Transparenz dokumentiert. | ✅ Documented |
| Public Sale Mode        | Public Sale ist der relevante aktuelle Zugangsmodus gemäß dokumentiertem Projektstatus.                     | ✅ Documented |
| Buy Limits              | Mindestkauf: `0.1 SOL`. Maximalkauf: `100 SOL`.                                                             | ✅ Passed     |
| Access Logic Separation | Kaufgrenzen und Legacy-Whitelist-Balance-Checks sind getrennte Mechanismen.                                 | ✅ Clarified  |
| Manual Whitelist        | Manuelle Whitelist-Kontrolle bleibt an die definierte autorisierte Projektrolle gebunden.                   | ✅ Passed     |
| Public Sale Toggle      | Public Sale kann gemäß definierter Programmlogik aktiviert oder deaktiviert werden.                         | ✅ Passed     |

### Clarification

The whitelist balance check and the minimum buy amount are not the same mechanism.

The minimum buy amount defines the required SOL contribution for a buy transaction.
The legacy whitelist balance check belongs to the earlier presale/access structure.

While public sale mode is active, public sale status is the relevant user access model. Legacy whitelist logic remains documented for transparency and should not be confused with current public sale buy limits.

This is not classified as a critical funds-at-risk issue. It is documented as a transparency and communication clarification so users can understand the difference between legacy whitelist logic, public sale mode and active buy limits.

---

## 🧾 8. Metadata & Transparency

| Datei                                  | Beschreibung                                                    | Ergebnis |
| -------------------------------------- | --------------------------------------------------------------- | -------- |
| `metadata.json` / `metadata.dmd2.json` | Enthält öffentliche Token-, Projekt- und Referenzinformationen. | ✅ Passed |
| `security.txt`                         | Enthält öffentlichen Sicherheitskontakt und Security-Referenz.  | ✅ Passed |
| `policy.html`                          | Enthält Disclaimer, Risiko- und Projektinformationen.           | ✅ Passed |
| `LICENSE`                              | MIT License vorhanden.                                          | ✅ Passed |
| `README.md`                            | Enthält Logo, Projektinfos, Referenzen, Status und Disclaimer.  | ✅ Passed |
| `index.html`                           | Öffentliche Public Reference Page für DMD vorhanden.            | ✅ Passed |

---

## 🧠 9. Security & Best Practices

| Test                   | Beschreibung                                                                                                 | Ergebnis |
| ---------------------- | ------------------------------------------------------------------------------------------------------------ | -------- |
| Anchor Version         | Anchor-Version ist im öffentlichen Projektkontext dokumentiert und build-kompatibel.                         | ✅ Passed |
| Solana Version         | Solana-Version ist im Build-/Verification-Kontext dokumentiert.                                              | ✅ Passed |
| Wallet Protection      | Private Wallet-Dateien, `.env`-Dateien und sensible Schlüssel sind nicht Teil des öffentlichen Repositories. | ✅ Passed |
| External Calls         | Keine unnötigen externen Programmaufrufe außerhalb definierter System-/Token-Programmlogik dokumentiert.     | ✅ Passed |
| Re-Initialization      | Doppelte Initialisierung wird durch definierte PDA-/Account-Logik verhindert.                                | ✅ Passed |
| Public-Safe Repository | Das Repository enthält public-safe Source-, IDL-, Policy-, Audit- und Security-Referenzen.                   | ✅ Passed |

---

## 🧪 10. Known Limitations / Review Notes

| Bereich             | Hinweis                                                                                                                      | Status         |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------- |
| External Audit      | Dieses Dokument ist ein Founder Pre-Audit Self-Check und ersetzt kein externes Security-Audit.                               | ⚠️ Recommended |
| Upgrade Authority   | Upgrade Authority ist laut öffentlichem Status nicht entfernt. Änderungen am Programm bleiben dadurch grundsätzlich möglich. | ⚠️ Transparent |
| Sell Status         | Sell Live ist laut dokumentiertem Status `false` und kann vom aktiven Systemzustand abhängen.                                | ✅ Documented   |
| Treasury Dependency | Treasury-, Vault-, Reward- und Sell-Funktionen können von Bestand, Authority und Systemzustand abhängen.                     | ✅ Documented   |
| Legacy Whitelist    | Legacy Self-Whitelist ist dokumentiert, aber nicht als aktueller Hauptzugang während Public Sale zu verstehen.               | ✅ Clarified    |

---

## ✅ Gesamtbewertung

**Audit Status:** 🟡 Public Pre-Audit Self-Check documented
**Core Status:** Core mechanics and public references reviewed in the founder self-check
**Risk Status:** No critical funds-at-risk issue identified in this checklist
**Recommendation:** Independent external code review remains recommended before presenting DMD as externally audited
**Letztes Update:** 2025-10-05

---

## Final Note

DMD provides public references, a verified build record, public-safe source material and a transparent checklist structure.

This checklist is intended to support transparency, documentation quality and public review. It should be read together with the README, Policy, Security.txt, Public Reference Page, IDL and current on-chain state.
