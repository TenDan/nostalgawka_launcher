# Nostalgawka Launcher

Minecraft Launcher built with Tauri (Rust Backend and Vite + Vue 3 configuration)

It will allow to run any Minecraft version specified by user with .jar file.

## Currently work in progress. Stay tuned.

Rust backend layer should access Java installed on the client system, settings stored in the place where application data is stored. 
Vue frontend handles user's interaction in the launcher (profile change/settings modification etc.)

Settings will include: 
- Java installation PATH (that should be automatically detected on the first startup of the app)
- Java arguments (e.g. -Xmx2G)

Profile settings will include:
- Minecraft version .jar archive that should be run.
- Name of the profile