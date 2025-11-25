# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


pnpm tauri signer generate -w app.key

Your keypair was generated successfully
Private: I:\updaterTauri\app.key (Keep it secret!)
Public: I:\updaterTauri\app.key.pub
---------------------------

Environment variables used to sign:
`TAURI_SIGNING_PRIVATE_KEY`  Path or String of your private key
`TAURI_SIGNING_PRIVATE_KEY_PASSWORD`  Your private key password (optional)

set | findstr TAURI


set TAURI_SIGNING_PRIVATE_KEY=dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5QW04dW56RUxUNWR4aU1LSFlKZzN6SlFJVTdMWDVUbXQ3a2RZbnBUdUEvSUFBQkFBQUFBQUFBQUFBQUlBQUFBQXdrRm1Vd2VwcENORmJJdTR3aFR4RzZQUS9tbUY0MnhXMFN0TDgyY0lOb1pGNWUwRXZDV1VxakpoUWRNK2tLWkNUR24rMFJNSnpUalBJcmtjM0NrWFJ4ME1sVURFcElSenZPUWs5YjRaMXozcG1ZR0M3K0cvelhCY0plL1AwL3Y0akJ3dWU4c01oQXM9Cg==

set TAURI_SIGNING_PRIVATE_KEY_PASSWORD=tanzania

pnpm tauri build