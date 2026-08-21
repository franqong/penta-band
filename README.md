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
*   **UI Framework Selection (Qt):** Create a very basic layout prototype using Qt.

### Phase 2: Core Implementation
*   **Markdown Lyric Editor:** Develop a custom widget for Markdown-based lyric editing with chord integration.
*   **Score Export Integration:** Implement functionality to export scores using LilyPond as a textual backend.
*   **Basic Playback Engine:** Integrate a basic audio playback engine (e.g., using RtAudio/FluidSynth for MIDI playback from generated scores).
*   **File Import/Export:** Enable import and export of relevant files (lyrics, PDF scores, MIDI files).

### Phase 3: Integrations & Improvements
*   **Open-Source DAW Integration:** Explore import/export of MIDI and audio with open-source DAWs.
*   **Enhanced Score Editing/Visualization:** Improve score editing and visualization (consider Musescore, Verovio, or custom Qt widgets).
*   **Project/Album Management:** Add features for structuring projects and albums with metadata.

### Phase 4: Future Online Preparation
*   **Modular Architecture:** Maintain a highly modular and well-documented codebase.
*   **Backend/UI Separation:** Consider separating backend processing/storage from the UI to facilitate future migration to a web platform.

## Technologies

Penta Band is committed to leveraging open-source tools to ensure a collaborative and accessible development environment.

*   **Graphical Framework:** Qt (for a robust, cross-platform user interface).
*   **Audio & MIDI:** RtAudio / RtMidi (for low-latency audio input/output and MIDI communication).
*   **Text/Music Processing:** LilyPond (for high-quality musical notation engraving and export).
*   **Language:** C++ (for performance and system-level control).
*   **Build System:** CMake (for cross-platform build process management).
*   **Version Control:** Git (for collaborative development).

## Key Development Advice

*   **Desktop-First Development:** Develop and thoroughly test the core functionalities on a powerful desktop environment.
*   **Laptop Usability:** Continuously test on a laptop to ensure the software remains usable and lightweight on hardware-limited systems.
*   **Modular Architecture:** Segment the application into distinct modules to allow loading only necessary components, optimizing performance on less powerful machines.
*   **CLI & Cross-Platform Tools:** Prioritize the use of Command Line Interface (CLI) and cross-platform tools (like CMake, Git, LilyPond) for broader compatibility and ease of development.

## Contributing

As an open-source project, contributions are highly welcome! Please refer to the `CONTRIBUTING.md` (to be created) for guidelines on how to get involved.
