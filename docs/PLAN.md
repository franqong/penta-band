# Plan tecnológico y de migración de Penta Band

## Objetivo

Penta Band será una aplicación de escritorio multiplataforma para composición
musical, letras, acordes, partituras y reproducción. El proyecto fue migrado
desde C++/Qt/CMake hacia Rust nativo con Cargo y GPUI, manteniendo el dominio
musical independiente de la interfaz.

Penta Band y Penta compartirán identidad visual, cuenta y contratos de datos
cuando corresponda, pero Penta Band debe funcionar localmente y sin depender de
la red para editar o guardar proyectos.

## Estado actual

La migración inicial ya está realizada:

- Workspace Cargo con `crates/`.
- `penta-band-desktop`: ventana GPUI con header, body vacío y footer.
- `penta-band-core`: modelo inicial de dominio.
- `penta-band-audio`: crate preparado para CPAL y midir mediante la feature
  opcional `native-audio`.
- `penta-band-markup`: `ropey` y `pulldown-cmark` incorporados.
- Recursos SVG embebidos desde `resources/`.
- Eliminados los archivos y la configuración de C++/Qt/CMake.

Todavía no están implementados el editor, la persistencia de proyectos, el
transporte de audio, MIDI funcional ni la integración con LilyPond.

## Decisiones tecnológicas

### Interfaz

La opción principal es **GPUI**, de Zed Industries:

- Rust nativo, sin JavaScript ni webview.
- Renderizado acelerado por GPU.
- Adecuado para una aplicación con paneles, comandos y estado compartido.

GPUI sigue siendo pre-1.0, por lo que se debe encapsular en
`penta-band-desktop` y evitar que el dominio dependa de sus tipos. Si aparecen
bloqueos importantes en edición de texto, accesibilidad, IME o plataformas
objetivo, evaluar **Iced** como alternativa principal. **Slint** queda como
alternativa declarativa, aunque agrega un lenguaje de interfaz propio.
**Tauri** no es la opción prevista porque mantiene la estructura de UI en
HTML/CSS/JavaScript.

### Audio y MIDI

- **CPAL**: acceso a dispositivos y streams de audio.
- **midir**: entrada y salida MIDI multiplataforma.
- **Rodio**: evaluar para previews sencillos, sin convertirlo en la capa de
  tiempo real.
- **Kira**: evaluar más adelante si se necesitan transporte, loops,
  automatización y control musical de alto nivel.
- **fundsp** o **dasp**: incorporar solo cuando exista una necesidad concreta
  de DSP.

La primera implementación debe separar reproducción, MIDI, render offline y
dispositivos. El callback de audio no debe bloquear, acceder al disco ni
realizar trabajo impredecible.

### LilyPond y notación

LilyPond no se reemplazará inicialmente. Se conservará como backend de
grabado/exportación porque ofrece una calidad difícil de reimplementar.

La integración futura debe:

1. Generar un modelo musical validado desde el dominio.
2. Convertirlo a un archivo `.ly` con escape correcto.
3. Ejecutar LilyPond como proceso aislado.
4. Capturar salida, errores, código de salida y archivos generados.
5. Exportar PDF/SVG y conservar el `.ly` reproducible.
6. Detectar si LilyPond está instalado y permitir configurar su ruta.

**Verovio** o MusicXML podrán evaluarse para previsualización interactiva,
pero no reemplazarán a LilyPond hasta demostrar equivalencia suficiente.

### Letras y Markdown

La base será:

- **ropey** para el buffer editable e índices de texto.
- **pulldown-cmark** para parsear Markdown.
- Sintaxis propia documentada para acordes y anotaciones musicales.

No se guardará HTML como fuente. El proyecto conservará Markdown y metadatos
estructurados. **comrak** se evaluará solamente si se necesita compatibilidad
GFM, AST mutable o transformaciones más complejas. Para resaltado incremental,
evaluar `tree-sitter-markdown` después de contar con un editor funcional.

## Arquitectura objetivo

El workspace crecerá de forma incremental:

```text
crates/
  penta-band-core/       # dominio y modelos serializables
  penta-band-project/    # formato, guardado, autosave y migraciones
  penta-band-markup/     # buffer, Markdown y sintaxis musical
  penta-band-score/      # modelo musical y exportadores
  penta-band-audio/      # CPAL, reproducción y render offline
  penta-band-midi/       # dispositivos y eventos MIDI
  penta-band-app/        # casos de uso y estado de aplicación
  penta-band-desktop/    # GPUI, ventanas y adaptadores de plataforma
```

La UI enviará comandos y observará estado; no accederá directamente al
sistema de archivos, procesos externos, audio ni red. Los procesos como
LilyPond estarán detrás de interfaces que puedan sustituirse por fakes en
tests.

## Formato de proyecto

El formato inicial será un directorio versionado y portable:

```text
song.pentaband/
  project.toml
  song.md
  score.ly
  audio/
  exports/
```

Debe incluir `format_version`, permitir migraciones explícitas y guardar de
forma atómica. El autosave y la recuperación se incorporarán antes de habilitar
edición compleja. El formato no debe depender del layout de GPUI.

## Roadmap de incorporación

### Etapa 1 — Base de aplicación

- Extraer comandos, estado y errores del desktop hacia crates independientes.
- Definir canciones, secciones, acordes, letras y metadatos en `core`.
- Implementar abrir, crear, guardar y reabrir proyectos.
- Incorporar serialización, `format_version`, migraciones y guardado atómico.
- Mantener header, body y footer como shell visual mientras se construye el
  estado real.

**Resultado:** crear un proyecto local, guardarlo y volver a abrirlo sin Qt.

### Etapa 2 — Editor de letras

- Crear el buffer editable con `ropey`.
- Añadir edición, selección, undo/redo y atajos.
- Parsear Markdown con `pulldown-cmark`.
- Definir y documentar la sintaxis de acordes.
- Renderizar una previsualización separada de la fuente Markdown.

**Resultado:** escribir y guardar una letra con acordes sin perder el formato.

### Etapa 3 — Notación y LilyPond

- Crear `penta-band-score`.
- Definir una representación musical intermedia.
- Generar `.ly` de forma determinista.
- Integrar detección y ejecución de LilyPond.
- Mostrar errores de exportación en la interfaz.
- Añadir exportación PDF/SVG y pruebas con fixtures.

**Resultado:** exportar una canción mínima reproducible sin depender de Qt.

### Etapa 4 — Audio y MIDI

- Implementar transporte, tempo, metrónomo y reproducción de preview.
- Activar `native-audio` solo cuando el backend nativo esté listo.
- Encapsular CPAL detrás de una interfaz de dispositivos y streams.
- Encapsular midir detrás de una interfaz de puertos y eventos.
- Manejar desconexiones y errores sin bloquear la interfaz.
- Añadir tests sin hardware y una prueba manual de latencia.

**Resultado:** reproducir un preview y recibir/enviar MIDI en dispositivos
soportados.

### Etapa 5 — Flujo de trabajo musical

- Integrar partituras, letras, acordes y múltiples canciones.
- Añadir álbumes, archivos recientes y recuperación de autosave.
- Completar importación/exportación MIDI y formatos interoperables.
- Mejorar accesibilidad, atajos, escalado y localización.

### Etapa 6 — Distribución e integración con Penta

- Empaquetar para Linux, Windows y macOS.
- Documentar LilyPond, fuentes y dependencias opcionales.
- Añadir sincronización con Penta sin acoplarla al núcleo local.
- Compartir contratos y tipos estables entre Penta Band y Penta.
- Integrar la cuenta común cuando exista el backend correspondiente.

## Reglas de desarrollo

- No introducir JavaScript, webviews ni una UI HTML para reemplazar GPUI.
- Mantener el dominio libre de GPUI, CPAL, midir y detalles de plataforma.
- No llamar al sistema de archivos, LilyPond o la red desde las vistas.
- Mantener una ruta CLI para validar proyectos y exportaciones sin abrir la UI.
- Añadir pruebas para serialización, migraciones, Markdown y generación `.ly`.
- Medir arranque, memoria, latencia, exportación y tamaño del paquete antes de
  declarar mejoras.
- Ejecutar builds, checks y tests de forma explícita durante el desarrollo,
  especialmente después de incorporar cada crate nuevo.
