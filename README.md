<p align="center">
  <img src="assets/lvs-logo.png" width="120" alt="LVS logo">
</p>

<h1 align="center">LVS — Living Value System</h1>

<p align="center">
  <b>Tasks nearby. Paid for proven results.</b><br>
  Someone close by does a real-world task, LVS proves where and when, the requester pays.
</p>

<p align="center">
  <a href="https://github.com/VOVAN1980/lvs-core/releases/latest/download/LVS-Android.apk"><img src="https://img.shields.io/badge/Download-Android-3DDC84?style=for-the-badge&logo=android&logoColor=white" alt="Download for Android"></a>
  <a href="https://github.com/VOVAN1980/lvs-core/releases/latest/download/LVS-Windows-x64.zip"><img src="https://img.shields.io/badge/Download-Windows-0078D4?style=for-the-badge&logo=windows&logoColor=white" alt="Download for Windows"></a>
  <a href="https://t.me/LVS_official"><img src="https://img.shields.io/badge/Telegram-LVS_official-26A5E4?style=for-the-badge&logo=telegram&logoColor=white" alt="Telegram channel"></a>
</p>

<p align="center">
  <a href="https://app.lvs.network">app.lvs.network</a> ·
  <a href="https://t.me/LVSNetworkBot">@LVSNetworkBot</a> ·
  <a href="mailto:support@lvs.network">support@lvs.network</a>
</p>

---

## What LVS does

A photo of a shop sign, a check on site, a short live video from a place — someone nearby does it, the app proves where and when it was done, and the person who asked pays.

- **Tasks near you.** Find tasks by country, region or town, or post your own with a deadline and a price.
- **Proof, not promises.** Photos and videos are taken by the LVS camera with time and GPS attached; live video from the spot, a GPS check-in, a file or a text are also possible. Faces and number plates can be hidden on request.
- **Fair outcomes.** Accepted work is paid at once. If the requester stays silent, the worker is paid automatically. A dispute is decided by a built-in judge from the evidence — place, time, media — not from words; paid tasks allow a short appeal to a human administrator.
- **Honest rating.** It changes only on real outcomes: a payout or a judge's decision.
- **VU** — the in-app unit for paid tasks: 1 VU = 1 USDT/USDC. Deposits and withdrawals go to your own crypto wallet on BNB Chain, Ethereum, Polygon, Base and TON.
- **BON** — bonus points to try LVS without money: 10 BON for confirming your identity in [@LVSNetworkBot](https://t.me/LVSNetworkBot), +1 for every invited friend. BON are never money.
- **Private by design.** Chats and media between the two sides of a task are encrypted; servers relay and coordinate, they do not keep your media.
- **30 languages**, with paid translation of chat and live video between the two sides.

## How it works

1. **Post** — describe the task, choose the place and the price in VU or BON.
2. **Do** — someone nearby takes it and delivers proof from the spot.
3. **Get paid** — the requester accepts, or the judge decides; the payout follows the result.

## Download

| Platform | File | Notes |
|---|---|---|
| Android 7+ (64-bit) | [LVS-Android.apk](https://github.com/VOVAN1980/lvs-core/releases/latest/download/LVS-Android.apk) | Allow installing from this source when Android asks. |
| Windows 10/11 (x64) | [LVS-Windows-x64.zip](https://github.com/VOVAN1980/lvs-core/releases/latest/download/LVS-Windows-x64.zip) | Unpack and run `LVS.exe`. If SmartScreen appears: *More info → Run anyway*. |
| Telegram | [@LVSNetworkBot](https://t.me/LVSNetworkBot) | Mini app for your account, BON and invitations. |
| iPhone | — | In the works. |

All versions and checksums: [Releases](https://github.com/VOVAN1980/lvs-core/releases).

## How it is built (in short)

LVS is a peer-to-peer network with one shared core (Rust) that runs on every device. Each action is a signed event, and every device checks the same rules, so no single server decides alone. Money movements are recorded in an append-only ledger with independent verification; keys that can move funds live only in a separate signing service, never in the application server.

## Status

LVS is in **test mode** on Android, Windows and Telegram. The official launch will start with a clean ledger: test balances are not carried over. The source code is not public.

## Contact

Questions and partnerships: [support@lvs.network](mailto:support@lvs.network), [@LVSNetworkBot](https://t.me/LVSNetworkBot) or an issue in this repository.

---

<p align="center">Built by <a href="https://vovan1980.github.io/IBITILabs/">IBITI Labs</a> · © 2025–2026 LVS. All rights reserved.</p>
