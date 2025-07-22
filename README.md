# Pyrefly Zed Extension

Una extensión para [Zed](https://zed.dev) que agrega soporte para el Language Server Protocol (LSP) de [Pyrefly](https://pyrefly.org/), un type checker rápido para Python.

## Características

- Integra Pyrefly LSP en Zed
- Proporciona verificación de tipos en tiempo real
- Soporte completo para las características del LSP (autocompletado, ir a definición, hover, etc.)

## Requisitos

- [uv](https://github.com/astral-sh/uv) instalado y disponible en tu PATH
- [pyrefly](https://pyrefly.org/) instalado (`uv pip install pyrefly`)

## Instalación

### Como extensión de desarrollo

1. Clona este repositorio
2. En Zed, abre la paleta de comandos (`Cmd+Shift+P` o `Ctrl+Shift+P`)
3. Ejecuta `zed: extensions`
4. Click en "Install Dev Extension"
5. Selecciona la carpeta del repositorio clonado

### Configuración

Una vez instalada la extensión, configura Python para usar Pyrefly en tu `settings.json`:

```json
{
  "languages": {
    "Python": {
      "language_servers": [
        "pyrefly",
        "!ruff",
        "!pyright"
      ]
    }
  }
}
```

## Desarrollo

Para compilar la extensión localmente:

1. Instala Rust via [rustup](https://rustup.rs/)
2. Agrega el target WebAssembly: `rustup target add wasm32-wasip2`
3. Compila con: `cargo build --target wasm32-wasip2`

## Licencia

MIT

## Contribuciones

¡Las contribuciones son bienvenidas! Por favor, abre un issue o pull request.
