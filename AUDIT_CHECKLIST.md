# 🔍 Die Mark Digital (DMD) – Smart Contract Audit Checklist

**Program ID:** `EDY4bp4fXWkAJpJhXUMZLL7fjpDhpKZQFPpygzsTMzro`  
**Token Standard:** SPL Token 2020  
**Network:** Solana Mainnet  
**Date:** 2025-10-05  
**Audited by:** DMD Founder (Pre-Audit Self-Check)

---

## 🧩 1. Overflow & Arithmetic Checks

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| `u64` / `BN` Berechnungen | Alle Rechenoperationen (SOL-Beiträge, Rewards, Penalties) werden in `u64` bzw. `anchor.BN` ausgeführt. Overflow-Checks sind in Release-Build aktiviert (`overflow-checks = true`). | ✅ Passed |
| Reward & Penalty Skalierung | Rewards und Penalties nutzen Prozent-basierte Multiplikation mit Division durch konstante Basiswerte (keine Division durch Variablen). | ✅ Passed |
| Treasury-Splits | Splits (60/40, 65/35) werden über feste Ganzzahl-Multiplikation durchgeführt; kein Rundungsfehler durch Float. | ✅ Passed |

---

## 🧾 2. Signer & Authority Validation

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Founder-Signaturen | Alle Founder-Operations (`initialize`, `set_manual_price`, `toggle_public_sale`, `whitelist_add`) verlangen Signer-Prüfung auf Founder Wallet `AqPFb5LWQuzKiyoKTX9XgUwsYWoFvpeE8E8uzQvnDTzT`. | ✅ Passed |
| Treasury Authority | Treasury (`CEUmazdgtbUCcQyLq6NCm4BuQbvCsYFzKsS5wdRvZehV`) wird nur bei `sell_dmd_v2` oder `swap_exact_dmd_for_sol` als Signer verwendet. | ✅ Passed |
| Buyer Auth | Käufertransaktionen (`buy_dmd`, `claim_reward`, `sell_dmd_v2`, `swap_exact_sol_for_dmd`) prüfen immer auf `buyer.is_signer`. | ✅ Passed |
| PDA-Ownership | Vault- und BuyerState-PDAs validieren Seeds (`vault`, `buyer`) gegen Program ID. | ✅ Passed |

---

## 💰 3. Treasury & Split Logic

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Presale-Split | 60 % Founder / 40 % Treasury (SOL-Einzahlungen im Presale). | ✅ Passed |
| Buy/Sell Fees | Buy-Fee 16.5 %, Sell-Fee 17.5 %; Split Founder 65 % / Treasury 35 %. | ✅ Passed |
| Reward Pool | Treasury finanziert Rewards ausschließlich aus Netto-Gebühren. Kein Mint-Inflation-Mechanismus. | ✅ Passed |
| Withdraws | Keine externe `withdraw()`-Funktion implementiert. Nur Founder-kontrollierte Treasury-Verwendung möglich. | ✅ Passed |

---

## 🏦 4. Vault / BuyerState Integrity

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| PDA Seeds | Vault-PDA: `[b"vault"]` → `AfbZG6WHh462YduimCUmAvVi3jSjGfkaQCyEnYPeXwPF` | ✅ Passed |
| BuyerState-PDA | `[b"buyer", vault, buyer]` korrekt implementiert. | ✅ Passed |
| Rent-Exemption | Accounts (`Vault`, `BuyerState`) werden mit `SystemProgram` & `RentExempt` erstellt. | ✅ Passed |
| Data Layout | Alignments: 8-Byte Discriminator + Struct-Felder (pubkey, u64, bool). Keine Padding-Fehler. | ✅ Passed |

---

## 🪙 5. Token Logic & Supply Control

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Mint Supply | 150 000 000 DMD initial über Founder-Wallet gemintet. | ✅ Passed |
| Decimals | `9` – standard SPL-Kompatibilität. | ✅ Passed |
| Token Standard | SPL Token 2020 – kein Legacy (2020 kompatibles Mint). | ✅ Passed |
| Vault-ATA | Vault besitzt eigenes ATA für DMD – Token-Transfers ausschließlich über Program Signer. | ✅ Passed |

---

## ⚙️ 6. Reward / Penalty Mechanism

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Reward v2 | `claim_reward_v2` nutzt echten SPL-Transfer Vault→Buyer. | ✅ Passed |
| Hold Duration | 30-Tage-Lock für Verkäufe (HOLD_DURATION). | ✅ Passed |
| Claim Interval | 90-Tage-Reward-Intervall (REWARD_INTERVAL). | ✅ Passed |
| Penalty Tier | Dynamische Staffelung: kleine Beträge 10 %, hohe 17.5 %. | ✅ Passed |

---

## 🧰 7. Whitelist & Public Sale

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Auto-Whitelist | `auto_whitelist_self` aktiv für Käufer ≥ 0.5 SOL. | ✅ Passed |
| Manual Whitelist | Nur Founder darf Whitelist-Status manuell setzen. | ✅ Passed |
| Public Sale Toggle | Founder kann Public Sale aktivieren/deaktivieren (`toggle_public_sale`). | ✅ Passed |

---

## 🧾 8. Metadata & Transparency

| Datei | Beschreibung | Ergebnis |
|--------|---------------|-----------|
| `metadata.json` | Enthält Mint, Vault, Treasury, Founder, Pool, Socials. | ✅ Passed |
| `security.txt` | Kontaktadresse + rechtlicher Disclaimer. | ✅ Passed |
| `policy.html` | Zweisprachiger Disclaimer (DE/EN) verfügbar via GitHub Pages. | ✅ Passed |
| `LICENSE` | MIT License aktiv. | ✅ Passed |
| `README.md` | Logo, Projektinfos, Socials & Disclaimer. | ✅ Passed |

---

## 🧠 9. Security & Best Practices

| Test | Beschreibung | Ergebnis |
|------|---------------|-----------|
| Anchor Version | 0.31.1 – stabiler Release mit IDL-Kompatibilität. | ✅ Passed |
| Solana Version | 2.3.0 – kompatibel zu SPL 2020. | ✅ Passed |
| Wallet Protection | Founder-/Treasury-Keys lokal gesichert (nicht in Repo). | ✅ Passed |
| External Calls | Keine Cross-Program Invocations außer System/Token Program. | ✅ Passed |
| Re-Initialization | Doppelte `initialize()` ausgeschlossen durch Vault-PDA Check. | ✅ Passed |

---

## ✅ Gesamtbewertung

**Audit Status:** 🟢 *Alle Core-Module funktionsfähig und sicher (Pre-Audit bestanden)*  
**Empfehlung:** Optionales externes Code-Review durch Anchor Security oder Helius Audit Services.  
**Letztes Update:** 2025-10-05  
