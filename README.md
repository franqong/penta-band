<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/branding/penta-band-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="assets/branding/penta-band-light.svg">
    <img src="assets/branding/penta-band-light.svg" width="200" alt="Penta Band">
  </picture>
</p>

Penta Band is a standalone, open-source desktop application designed as a dedicated workspace for music composition. It serves as a complementary project to "Penta," a web-based social music network, aiming to provide composers with a comprehensive toolset for their creative process, from initial ideas to finished pieces. The long-term vision includes deep integration with the Penta social platform, allowing seamless sharing and collaboration.

## Project Vision

The core idea behind Penta Band is to offer a robust and lightweight environment where musicians can:
*   **Write Music:** Create and edit musical scores.
*   **Write Lyrics:** Develop and organize lyrical content.
*   **Integrate with DAWs:** (Future) Connect with Digital Audio Workstations for advanced production.
*   **Connect with Penta:** (Future) Seamlessly interact with the Penta social music network.

It's built with the composer's needs in mind, providing all the essential tools required for the creative journey.

## Key Features (Planned Roadmap)

### Phase 1: Core Design & Tool Selection
*   **Minimal Functionality & Workflow Definition:** Outline essential features and user flow (e.g., "New Project / Song / Album").
*   **Text Editor for Lyrics:** Implement a Markdown-based text editor.
*   **Chord/Note Annotation:** Ability to add chords and musical notes over lyrics.
*   **Simple Score Editor:** A basic editor with export capabilities to PDF/SVG via LilyPond.
*   **Native Rust UI:** Build the desktop shell with GPUI and keep the domain
    independent from the interface.

### Phase 2: Core Implementation
*   **Markdown Lyric Editor:** Develop a custom widget for Markdown-based lyric editing with chord integration.
*   **Score Export Integration:** Implement functionality to export scores using LilyPond as a textual backend.
*   **Basic Playback Engine:** Integrate CPAL for audio devices and midir for
    MIDI communication.
*   **File Import/Export:** Enable import and export of relevant files (lyrics, PDF scores, MIDI files).

### Phase 3: Integrations & Improvements
*   **Open-Source DAW Integration:** Explore import/export of MIDI and audio with open-source DAWs.
*   **Enhanced Score Editing/Visualization:** Improve score editing and
    visualization using Rust-native components and LilyPond/Verovio where
    appropriate.
*   **Project/Album Management:** Add features for structuring projects and albums with metadata.

### Phase 4: Future Online Preparation
*   **Modular Architecture:** Maintain a highly modular and well-documented codebase.
*   **Backend/UI Separation:** Consider separating backend processing/storage from the UI to facilitate future migration to a web platform.

## Technologies

Penta Band is committed to leveraging open-source tools to ensure a collaborative and accessible development environment.

*   **Language:** Rust.
*   **Graphical Framework:** GPUI, a GPU-accelerated native Rust UI framework.
*   **Workspace:** Cargo workspace organized into independent crates:
    `penta-band-core`, `penta-band-desktop`, `penta-band-audio` and
    `penta-band-markup`.
*   **Audio & MIDI:** CPAL and midir, enabled as the native audio layer is
    implemented.
*   **Lyrics and Markdown:** ropey for text editing and pulldown-cmark for
    Markdown parsing and rendering.
*   **Music notation:** LilyPond as the initial engraving and export backend.
*   **Assets:** SVG resources stored in `resources/icons/` and embedded into
    the desktop binary.
*   **Build System:** Cargo.
*   **Version Control:** Git (for collaborative development).

## Key Development Advice

*   **Desktop-First Development:** Develop and thoroughly test the core functionalities on a powerful desktop environment.
*   **Laptop Usability:** Continuously test on a laptop to ensure the software remains usable and lightweight on hardware-limited systems.
*   **Modular Architecture:** Segment the application into distinct modules to allow loading only necessary components, optimizing performance on less powerful machines.
*   **CLI & Cross-Platform Tools:** Prioritize Cargo, Git and LilyPond for
    reproducible development and broader compatibility.

> [!NOTE]
> Penta Band was migrated from C++/Qt to Rust/GPUI. The migration preserves the original desktop-first direction while introducing a modular Rust workspace for the interface, domain, audio, MIDI and markup layers.
