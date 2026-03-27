# Mise à jour du compilateur Flux# - Support Float et Double

## Résumé des changements

Vous avez demandé d'ajouter le support pour les types `float` et `double` avec la syntaxe suivante:
- **Float**: Les littéraux floats se terminent par `f` ou `F` (ex: `3.14f`, `2.5F`)
- **Double**: Les littéraux doubles utilisent la notation décimale (ex: `3.14`, `2.71828`)

## Modifications apportées

### 1. Grammaire PEST (`flux_grammar.pest`)

#### Avant
```pest
float_literal = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT* }
double_literal = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT* ~ ("e" | "E") ~ ("+" | "-")? ~ ASCII_DIGIT+ }
int_literal = @{ hex_literal | bin_literal | oct_literal | dec_literal }
```

#### Après
```pest
float_literal = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT* ~ ("f" | "F") }
double_literal = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT* ~ (("e" | "E") ~ ("+" | "-")? ~ ASCII_DIGIT+)? }
int_literal = @{ hex_literal | bin_literal | oct_literal | dec_literal }
```

**Changements**:
- Float maintenant NÉCESSITE le suffixe `f` ou `F`
- Double permet la notation scientifique optionnelle
- Ordre de reconnaissance mis à jour dans `atom` pour reconnaître float avant double

### 2. Système de types (`FluxType` enum)

Ajout des variantes:
```rust
pub enum FluxType {
    Int, UInt, Long, ULong, Byte, String, Bool, Void, Float, Double,
    Pointer(Box<FluxType>),
    Struct(String),
}
```

### 3. Valeurs runtime (`FluxValue` enum)

Ajout du support pour stocker les valeurs floats et doubles:
```rust
pub enum FluxValue {
    Integer(i64),
    Float(f32),
    Double(f64),
    Str(String),
}
```

### 4. Évaluateur d'expressions

Ajout du support pour:
- **Parsing des littéraux**: Conversion de `3.14f` → `f32` et `3.14` → `f64`
- **Opérations arithmétiques**: Support pour +, -, *, /, % sur float/double
- **Conversions implicites**: Float + Integer → Float, Double + Float → Double, etc.

### 5. Génération de code assembleur

Pour éviter les limitations NASM avec les immédiates flottantes:
- Les valeurs float/double sont stockées dans la section `.data`
- Les instructions `mov` chargent les bits depuis la mémoire
- Les bits sont encodés en hexadécimal (ex: `0x3f4ccccd` pour 3.14f)

Exemple généré:
```asm
float_0: dd 0x3f4ccccd    ; Stockage de 3.14f
mov eax, [rel float_0]     ; Charger les bits
mov [rbp-8], eax           ; Stocker sur la stack
```

### 6. Runtime (`runtime.asm`)

Ajout de deux fonctions de print stub:
```asm
global _fsh_print_float
_fsh_print_float:
    ; Affiche "[float]" pour l'instant
    lea rdi, [rel float_msg]
    call _fsh_print_str
    ret

global _fsh_print_double
_fsh_print_double:
    ; Affiche "[double]" pour l'instant
    lea rdi, [rel double_msg]
    call _fsh_print_str
    ret
```

**Note**: Ces fonctions affichent actuellement `[float]` et `[double]`. Pour implémenter la conversion complète en décimal, il faudrait ajouter une fonction de conversion flottant → chaîne ASCII (complexe en assembleur pur).

### 7. Chemin du runtime

Correction du chemin d'accès au `runtime.asm`:
- Avant: `"fluxc/runtime/runtime.asm"`
- Après: `"flux_compiler/fluxc/runtime/runtime.asm"`

Cela permet au compilateur de trouver le runtime correctement peu importe le répertoire d'exécution.

## Exemples d'utilisation

```flux
public static void Main() {
    // Floats (32-bit)
    float pi = 3.14159f;
    float e = 2.71828f;
    float ratio = 1.618f;
    
    // Doubles (64-bit)
    double sqrt2 = 1.41421356;
    double ln10 = 2.302585093;
    
    // Opérations arithmétiques
    float sum = 1.5f + 2.5f;      // = 4.0f
    double product = 2.0 * 3.0;   // = 6.0
    float quotient = 10.0f / 3.0f; // ≈ 3.333f
    
    // Affichage
    print(pi);        // Affiche "[float]"
    print(sqrt2);     // Affiche "[double]"
}
```

## Tests

Deux fichiers de test ont été créés:
- `test_floats.fsh`: Test basique avec quelques variables float/double
- `test_floats_complete.fsh`: Test complet avec diverses opérations

Compilation et exécution:
```bash
./flux_compiler/target/debug/fluxc compile test_floats.fsh -o test_floats.bin
./test_floats.bin
```

## Améliorations futures

1. **Conversion flottant → ASCII**: Implémenter une fonction assembly pour afficher les vraies valeurs
2. **Support FPU (x87 ou SSE)**: Utiliser les registres flottants pour de meilleures performances
3. **Constantes mathématiques**: Ajouter des constantes comme `PI`, `E`, etc.
4. **Fonctions mathématiques**: sin(), cos(), sqrt(), pow(), etc.
5. **Litéraux exponentiels complets**: `1.5e-3`, `2.0E+10`, etc.

