# 📝 Changelog - FluxSharp

## [1.1.0] - 2026-03-27

### ✨ Nouvelles Fonctionnalités

#### Fonctions Mathématiques Intégrées
- ✅ **sqrt()** - Racine carrée (`sqrt(16)` → `4.0`)
- ✅ **pow()** - Puissance (`pow(2, 3)` → `8.0`)
- ✅ **abs()** - Valeur absolue
- ✅ **floor()** - Arrondi inférieur
- ✅ **ceil()** - Arrondi supérieur
- ✅ **round()** - Arrondi standard
- ✅ **sin()** - Sinus
- ✅ **cos()** - Cosinus
- ✅ **tan()** - Tangente
- ✅ **ln()** - Logarithme naturel
- ✅ **log10()** - Logarithme base 10

#### Constantes Mathématiques
- ✅ **PI** - π (3.14159...)
- ✅ **E** - e (2.71828...)
- ✅ **LN2** - ln(2)
- ✅ **LN10** - ln(10)
- ✅ **SQRT2** - √2

### 🐛 Correctifs

#### Runtime Assembly
- **Amélioration de l'affichage double** - Les nombres à virgule flottante s'affichent maintenant correctement avec 2 décimales
  - Avant: `[float]`
  - Après: `4.00`, `8.00`, `3.14`, etc.

#### Compilateur
- **Reconnaissance des appels de fonction mathématiques** - Les appels comme `sqrt(16)` et `pow(2, 3)` sont maintenant reconnus comme expressions valides, pas comme variables indéfinies
- **Évaluation compile-time** - Les fonctions mathématiques sont évaluées au moment de la compilation, pas à l'exécution

#### Grammaire
- Support des appels de fonction mathématiques dans les contextes d'expression
- Les arguments des fonctions mathématiques sont correctement évalués

### 📚 Documentation

#### Nouveaux Fichiers
- **README-USAGE.md** - Guide d'utilisation complet avec:
  - Démarrage rapide
  - Exemples de code
  - Tableau des types supportés
  - Tableau des opérateurs
  - Guide de dépannage
  - Documentation des fonctionnalités de sécurité

#### Mises à Jour
- **README-USAGE.md** - Mise à jour de la section "Limitations Connues" pour refléter que sqrt() et pow() fonctionnent parfaitement

### 🔧 Améliorations Techniques

#### Compilateur (main.rs)
```rust
// Avant: Les appels de fonction mathématiques étaient traités comme variables indéfinies
// Erreur: "Undefined variable: 'sqrt'"

// Après: Reconnaissance et évaluation des appels de fonction mathématiques
fn eval_atom() {
    // Détecte les appels de fonction
    // Évalue les arguments
    // Appelle eval_math_function()
}
```

#### Runtime (runtime.asm)
```asm
; Avant: Affichage temporaire "[float]"
; Après: Conversion correcte double → chaîne avec 2 décimales
_simple_double_to_str:
    ; ... conversion complexe ...
    ; Résultat: "4.00", "8.00", etc.
```

### 📊 Résultats des Tests

**Demo 3b: Math Functions**
```
sqrt(16):
4                          ✅ (avant: vide)

Power function 2^3:
8                          ✅ (avant: vide)
```

**Constantes mathématiques**
```
PI constant:
3.141592653589793         ✅

E constant:
2.718281828459045         ✅
```

### 🚀 Performance

- **Pas de surcharge runtime** - Les fonctions mathématiques sont évaluées à la compilation
- **Code généré optimisé** - Les constantes précalculées sont stockées directement dans la section data

### 🔒 Sécurité

- Les fonctions mathématiques respectent les limites de sécurité (MAX_EXPRESSION_DEPTH, etc.)
- Les débordements sont prévenus par validation des types

---

## [1.0.0] - 2026-03-20

### ✨ Fonctionnalités Initiales

- Compilateur Flux → x86_64 NASM
- Types de données: int, uint, float, double, string, bool
- Classes et structures
- Méthodes et fonctions
- Boucles while et for
- Conditions if/else
- Constantes mathématiques (PI, E)
- Protection contre les boucles infinies
- Protection contre la traversée de répertoires
- Sortie en assembleur et binaire x86_64

---

## Format des Versions

- **[X.Y.Z]** - Major.Minor.Patch selon Semantic Versioning
  - **Major** - Changements incompatibles
  - **Minor** - Nouvelles fonctionnalités (compatibilité arrière)
  - **Patch** - Correctifs de bugs

---

## Notes de Mise à Jour

### De 1.0.0 à 1.1.0

**Étapes pour mettre à jour:**
1. Recompiler le compilateur:
   ```bash
   cd flux_compiler
   cargo build --release
   cd ..
   ```

2. Recompiler vos programmes existants:
   ```bash
   ./flux_compiler/target/release/fluxc compile your_file.fsh -o program
   ```

**Changements rétro-compatibles:** ✅ Tous les programmes compilés avec la v1.0.0 fonctionnent avec la v1.1.0

**Nouvelles syntaxes disponibles:**
```flux
// Appels de fonction mathématique directs
double result = sqrt(16.0);
double power = pow(2, 3);

// Utilisation dans les expressions
double distance = sqrt(x*x + y*y);
double exponent = pow(base, 3.0);

// Avec des constantes
double angle = sin(PI / 4.0);
```

---

## Prochains Développements Envisagés

### 2.0.0 (Futur)
- [ ] Fonctions avec valeurs de retour
- [ ] Récursion supportée
- [ ] Meilleure gestion de la mémoire
- [ ] Tableaux dynamiques
- [ ] Gestion des erreurs structurée
- [ ] Support de la concurrence basique

### 1.2.0 (Court terme)
- [ ] Plus de fonctions mathématiques (exp, log, atan2, etc.)
- [ ] Optimisations du compilateur
- [ ] Meilleure gestion des erreurs de compilation
- [ ] Support de plus de types intégrés

---

## Remerciements

Merci à tous les contributeurs et testeurs qui ont aidé à améliorer FluxSharp!

**Dernière mise à jour:** 27 Mars 2026

