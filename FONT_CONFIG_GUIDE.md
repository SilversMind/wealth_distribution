# Configuration des Tailles de Polices

## Résumé des modifications

Toutes les tailles de polices de l'application sont maintenant centralisées dans un fichier de configuration TOML.

## Structure

### Fichier de configuration
- **Chemin**: `config/font_sizes.toml`
- **Format**: TOML (Tom's Obvious, Minimal Language)

### Tailles de polices disponibles

| Paramètre | Défaut | Utilisation |
|-----------|--------|-------------|
| `agent_label` | 14.0 | Texte d'affichage de la richesse des agents |
| `main_title` | 24.0 | Titre principal (compteur de ticks) |
| `section_title` | 16.0 | Titres des sections (Richesse, Vitesse) |
| `button_text` | 16.0 | Texte des boutons de contrôle de vitesse |
| `legend_value` | 24.0 | Valeurs de la légende de richesse |

## Comment modifier les tailles

Ouvrez le fichier `config/font_sizes.toml` et ajustez les valeurs:

```toml
[font_sizes]
agent_label = 16.0   # Augmentez pour rendre les labels plus visibles
main_title = 28.0    # Augmentez le titre principal
section_title = 18.0
button_text = 18.0
legend_value = 26.0
```

Les modifications prennent effet au prochain redémarrage de l'application.

## Implémentation technique

### Structure Rust
Une struct `FontSizes` a été créée pour charger et gérer les tailles de polices:

```rust
struct FontSizes {
    agent_label: f32,
    main_title: f32,
    section_title: f32,
    button_text: f32,
    legend_value: f32,
}
```

### Chargement
La méthode `FontSizes::load()` lit le fichier de configuration au démarrage.
En cas d'erreur de lecture, les valeurs par défaut sont utilisées.

### Utilisation dans le code
La struct `fonts` est passée à la fonction `draw_world()` et utilisée pour tous les appels à `draw_text()`.

## Dépendances ajoutées
- **toml 0.5**: Parseur TOML pour Rust

## Fichiers modifiés
1. `src/main.rs` - Code principal (ajout de la struct et du chargement)
2. `Cargo.toml` - Ajout de la dépendance `toml`
3. `config/font_sizes.toml` - Fichier de configuration (créé)
