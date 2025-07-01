# LoL Term

### League of Legends Profile Display in Terminal

**LoL Term** is a simple Rust-based terminal application that fetches and displays a League of Legends summoner’s
profile information right in your terminal. It shows the summoner’s:

- Profile icon (rendered as an image or ASCII art in supported terminals)
- Summoner name
- Summoner level

This tool uses Riot Games’ official API to retrieve up-to-date profile data and the Data Dragon CDN to download summoner
icons.

---

### Features

- Fetch summoner profile by Riot ID and Tagline
- Download and cache profile icons locally
- Display profile icon scaled appropriately in the terminal
- Show summoner name and level alongside the icon
- Supports asynchronous API requests for responsive operation

---

### Planned Features

- Configuration support to select which account/profile to display
- Match history and ranked stats display
- Custom themes and layouts
- Better fallback for terminals without image support

---

### Usage

Run the app with your Riot API key and summoner information to instantly see your profile details in a compact terminal
view.

---

### Technologies

- Rust
- Reqwest for async HTTP requests
- Tokio async runtime
- Viuer for terminal image display
- Serde for JSON deserialization

---

> Display your League profile without leaving the command line!
