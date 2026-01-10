v0.0.1
- [x] - Make rendering via wgpu and make a working Elm architecture
      
v0.0.2
- [ ] - Optimize GlazeUI, currently during redraw, layout via taffy recompiles every time and for each text widget, a font is created, which leads to lag (to fix this, need to create the font only once and then use it)

v0.0.3
- [ ] - Add essential functions for widgets, such as align, padding, etc.

v0.1.0
- [ ] - Gradient support
- [ ] - Clipping

v0.1.1
- [ ] - Custom fonts
- [ ] - Font fallback
- [ ] - Font cache API

v0.1.2
- [ ] - API for custom widgets
- [ ] - Stable widget trait
- [ ] - Documentation

v0.2.0
- [ ] - Diagnostic API
- [ ] - Animations (Limited)
- [ ] - Async Support

v0.3.0
- [ ] - Add Skia Backend

v0.3.1
- [ ] - Mobile support via Skia (iOS - Metal, Android - OpenGL
- [ ] - Web Support