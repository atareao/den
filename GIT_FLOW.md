# Git Flow

Este proyecto sigue **Git Flow** con versionado semántico automático.

## Ramas

| Rama | Propósito | Base |
|---|---|---|
| `main` | Producción. Cada merge aquí dispara una release automática. | — |
| `development` | Integración de features en curso. | `main` |
| `feature/*` | Nuevas funcionalidades. | `development` |
| `hotfix/*` | Correcciones urgentes a producción. | `main` |

## Flujo diario

### Features

```bash
git checkout development && git pull origin development
git checkout -b feature/mi-feature
git commit -m "✨ feat: add dark mode toggle"
git push origin feature/mi-feature
# Crear Pull Request en GitHub
```

### Hotfixes

```bash
git checkout main && git pull origin main
git checkout -b hotfix/arreglo-critico
git commit -m "🚑️ hotfix: crash on empty input"
git push origin hotfix/arreglo-critico
# Crear Pull Request en GitHub
# Después del merge, sincronizar development
git checkout development && git merge main && git push origin development
```

### Releases

```bash
# PR: development → main
# Al mergear, CI automáticamente:
#   a) Detecta bump type según conventional commits
#   b) vampus actualiza versión
#   c) git-cliff genera CHANGELOG.md
#   d) Crea tag vX.Y.Z
#   e) GitHub Actions compila y publica
```

## Conventional Commits con Gitmoji

El formato determina el bump automático:

| Mensaje | Bump |
|---|---|
| `✨ feat: ...` | minor (0.Y.0) |
| `🐛 fix: ...` | patch (0.0.Z) |
| `♻️ refactor: ...` | patch (0.0.Z) |
| `📝 docs: ...` | patch (0.0.Z) |
| `💥 feat!: ...` o `BREAKING CHANGE` | major (X.0.0) |

### Gitmoji de referencia

| Tipo | Emoji |
|---|---|
| feat | ✨ |
| fix | 🐛 |
| hotfix | 🚑️ |
| docs | 📝 |
| refactor | ♻️ |
| perf | ⚡ |
| style | 💄 |
| test | ✅ |
| chore | 🔧 |
| ci | 👷 |
| revert | ⏪️ |
| deps | ⬆️ |
| breaking | 💥 |

## CI Workflows

- **`ci.yml`** — PR a main/development: formato, lint, build, tests
- **`release-prepare.yml`** — push a main: bump version, changelog, tag, sincroniza development
- **`release.yml`** — tag v*: compila multi-plataforma, publica en crates.io, GitHub Release

## Secretos de GitHub necesarios

| Secreto | Propósito |
|---|---|
| `GH_PAT` | Personal Access Token con scope `contents: write` |
| `CARGO_REGISTRY_TOKEN` | Token de API de crates.io |

## Resumen visual

```
main        ──hotfix──●────────────●────────────────●
                       \            /                /
development ──────────●──●──●────●──●──●──●────────●
                        \ /        \   /
feature     ──────────●  feature ──●
                      feature/foo   feature/bar

● = merge a development (PR)
● = merge a main (release automática)
```
