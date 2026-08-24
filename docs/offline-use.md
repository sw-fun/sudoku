# Offline Use and Phone Installation

Learn/Practice Sudoku is a Progressive Web App (PWA): it can be
installed on a phone like a native app and, once installed, it works
offline. The live app is served at
`https://sw-fun.github.io/sudoku/` over HTTPS, which is what enables
the service worker and installability.

## Installing on an iPhone or iPad (Safari)

1. Open `https://sw-fun.github.io/sudoku/` in Safari.
2. Tap the Share button (the square with an arrow) in the toolbar.
3. Scroll and tap **Add to Home Screen**.
4. Confirm the name (Learn/Practice Sudoku) and tap **Add**.

The app now appears on the home screen with its own icon. Launching it
opens a standalone window without the Safari address bar.

## Installing on Android (Chrome)

1. Open `https://sw-fun.github.io/sudoku/` in Chrome.
2. Open the menu (the three dots) and tap **Install app** -
   or tap the Install banner if Chrome offers one.
3. Confirm the install prompt.

The app appears in the app drawer and on the home screen, launching in
its own window.

## What works offline

- The board, lessons, show-me walkthroughs, and number input are all
  computed on-device in WebAssembly; no network is needed to play.
- The first load of each new app version must be online so the service
  worker can cache the shell (HTML, CSS, JS, WASM, icons). After that,
  the app opens and plays with no connection.
- Game progress and win counts are saved in the browser's local
  storage on every change, offline or online. One save slot holds the
  current board (one board at a time), and the menu offers
  **Resume** for an in-progress game.

## How updates arrive

Each release stamps a new service-worker cache version. Navigations
are fetched network-first, so a reload while online picks up a new
release immediately; the old cache is discarded once the new one is
active. A reload while offline serves the cached copy.

## Troubleshooting

- **Fresh install shows no saved game:** the save lives in the browser
  that installed the app. Installing from Safari and from Chrome keeps
  two separate saves, and using private/incognito mode does not
  persist at all.
- **Clearing site data** (browser settings, or iOS "Clear History and
  Website Data") erases the save slot and the offline cache together.
- **An old version will not go away:** the service worker updates on
  reload; if it refuses, unregister the site's data in browser
  settings or reinstall the home-screen app.
