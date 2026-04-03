# FluxSharp Compiler - Main Class Validation & Include System

## Overview

Le compilateur FluxSharp (fluxc) applique maintenant les vérifications suivantes:

### 1. **Main Class & Method Validation**

Chaque programme doit avoir:
- ✅ Exactement **UNE classe Main**
- ✅ Exactement **UNE méthode main()** dans cette classe

#### Format Requis:
```fsh
public class Main {
    public void main() {
        print("Hello, World!");
    }
}
```

#### Erreurs Détectées:
- ❌ **MISSING MAIN CLASS**: Aucune classe Main trouvée
- ❌ **MULTIPLE MAIN CLASSES**: Plusieurs classes Main déclarées
- ❌ **MISSING MAIN METHOD**: Pas de méthode main() dans la classe Main
- ❌ **MULTIPLE MAIN METHODS**: Plusieurs méthodes main() déclarées

### 2. **Include System for External .fsh Files**

Vous pouvez maintenant inclure des fichiers `.fsh` externes en utilisant:

```fsh
// #include "filename.fsh"
```

#### Règles d'Inclusion:
1. **Seuls les fichiers `.fsh`** sont autorisés
2. Les inclusions sont traitées **récursivement** (un fichier inclus peut inclure d'autres fichiers)
3. **Protection contre les inclusions circulaires** automatique
4. Les fichiers doivent être dans le **même répertoire** ou dans les sous-répertoires (chemins relatifs)

#### Exemple:
```fsh
// #include "helper.fsh"
// #include "math_utils.fsh"

public class Main {
    public void main() {
        print("Program with includes");
    }
}
```

#### Erreurs d'Inclusion:
- ❌ **INVALID INCLUDE FILE**: Le fichier n'a pas l'extension `.fsh`
- ❌ **INCLUDE FILE NOT FOUND**: Le fichier spécifié n'existe pas
- ❌ **CIRCULAR INCLUDE**: Une inclusion circulaire a été détectée

### 3. **Compilation Workflow**

```bash
# Compiler un fichier unique
cargo run --release -- compile main.fsh -o program

# Compiler tous les fichiers .fsh d'un répertoire
cargo run --release -- compile --all ./examples -o program

# Compiler et exécuter
cargo run --release -- compile main.fsh -o program --run
```

### 4. **Security Features**

- ✅ Validation des chemins d'accès (pas de path traversal)
- ✅ Limite de taille de fichier (50 MB max)
- ✅ Protection contre les inclusions circulaires
- ✅ Validation de la taille d'ASM générée (100 MB max)
- ✅ Limite de nombre de déclarations (10 000 max par bloc)
- ✅ Limite de profondeur d'expression (100 max)

## Example Project Structure

```
project/
├── main.fsh                 # Fichier principal avec la classe Main
├── helper.fsh              # Fichier helper (optionnel)
├── utils.fsh               # Fichier utilitaire (optionnel)
└── subdir/
    └── database.fsh        # Inclusion supportée
```

### main.fsh:
```fsh
// #include "helper.fsh"
// #include "subdir/database.fsh"

public class Main {
    public void main() {
        print("Program started");
        // Utiliser les classes définies dans helper.fsh
    }
}
```

## Error Messages

Le compilateur fournit des messages d'erreur clairs:

```
❌ MISSING MAIN CLASS

Your program must have exactly one 'class Main' with a 'void main()' method.

Example:
public class Main {
    public void main() {
        print("Hello, World!");
    }
}
```

## Implementation Details

### validate_main_class()
Vérifie que le code compilé contient:
- Exactement une déclaration `class Main`
- Exactement une déclaration `void main()`

### process_includes()
Traite les directives d'inclusion (`// #include "..."`) avant la compilation:
1. Scanne les lignes pour les directives d'inclusion
2. Valide que c'est un fichier `.fsh`
3. Charge et valide le fichier externe
4. Traite les inclusions imbriquées
5. Combine le contenu pour la compilation

## Notes

- Les commentaires d'inclusion doivent être au format: `// #include "filename.fsh"`
- Les inclusions sont traitées dans l'ordre d'apparition
- Les fichiers inclus doivent avoir une structure de classe valide
- Un fichier inclus ne doit pas contenir de classe `Main` (sinon erreur de classe Main multiple)

