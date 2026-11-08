Punchy 0.1.8 adds an interactive live in-game HUD editor, overhauled mod settings, and launcher build fixes:

- **Interactive In-Game Live HUD Editor (F9)**: press **F9** while playing in any world to display an on-screen translucent overlay. Switch sections with `Tab`, select parameters with `↑`/`↓`, and adjust values live with `←`/`→` or `-`/`=` to watch your character, camera, and arms shift in real time!
- **Overhauled Settings Screen**: under **Options > Mods > Punchy Rust > Settings**, all 4 parameters per category are displayed simultaneously with dedicated `[-]` and `[+]` stepping buttons and exact numerical feedback.
- **Horizon Native Build Compatibility**: fixed mixin helper function syntax and registered `native_gui_render_post` hook for zero build errors.
- **Punchy! Moves integration**: third-person dynamic shoulder camera, free / orbit camera (`F8`), stationary turn-in-place easing, and full viewmodel customization.

Download `punchy-rust-preview-0.1.8.hnmod`, add it to your instance's native mods, and rebuild the game in the launcher.

Requires Horizon Nightly 2026-11-05 (`66f3981`), Minecraft 26.3.

Based on Punchy and Punchy! Moves by Dev Punchy Man. Original license notices included.
