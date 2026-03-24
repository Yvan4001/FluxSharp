# 🚀 FluxGridOS Flux Language - Quick Start Guide

## ⚡ Démarrage Rapide (5 minutes)

### 1. Compiler les Fichiers Flux

```bash
cd /mnt/linux_plus/projet_dev/flux_grid_os
chmod +x build_flux_multi.sh
./build_flux_multi.sh
```

**Attendu:**
```
[INFO] Starting multi-file Flux compilation...
[INFO] Source files:
  ✓ ./os_fs/src/main.fsh
  ✓ ./os_fs/src/lib.fsh
  ✓ ./os_fs/src/async_example.fsh
[INFO] Compiling with fluxc...
✓ Compilation successful!
[INFO] Output file: ./os_fs/build/kernel.bin
```

### 2. Vérifier le Résultat

```bash
ls -lh os_fs/build/kernel.bin
# Affiche la taille du binaire compilé
```

### 3. Consulter la Documentation

```bash
# Aperçu général
cat guides/IMPLEMENTATION_COMPLETE.md

# Guide async/await
cat guides/ASYNC_AWAIT_GUIDE.md

# Système de compilation
cat guides/COMPILATION_SYSTEM.md
```

---

## 📖 Documentation par Besoin

### "Je veux comprendre Flux"
➜ Lire: `guides/FLUX_LANGUAGE_SUMMARY.md`
- Quels types sont supportés?
- Quelle syntaxe pour les boucles?
- Comment déclarer des structures?

### "Je veux utiliser async/await"
➜ Lire: `guides/ASYNC_AWAIT_GUIDE.md`
- Créer une fonction async
- Utiliser await
- Task management
- Exemples complets

### "Je veux compiler plusieurs fichiers"
➜ Lire: `guides/COMPILATION_SYSTEM.md`
- Comment ça fonctionne?
- Linking automatique
- Résolution de symboles
- Commands de compilation

### "Je veux voir du code d'exemple"
➜ Regarder: `os_fs/src/`
- `main.fsh` - Boot sequence
- `lib.fsh` - Kernel et async runtime
- `async_example.fsh` - Examples async
- `test.fsh` - Tests grammaire

---

## 🎯 Cas d'Usage Courants

### 1. Créer une Fonction Simple

**Fichier:** `os_fs/src/lib.fsh`

```flux
public void my_function() => {
    serial_print("Hello from Flux!");
}
```

**Compiler:**
```bash
./build_flux_multi.sh
```

---

### 2. Créer une Fonction Async

**Fichier:** `os_fs/src/lib.fsh`

```flux
public async uint fetch_data() => {
    uint task = async_create_task();
    async_schedule_task(task);
    return task;
}

public async void use_async() => {
    uint result = await fetch_data();
}
```

---

### 3. Utiliser Async/Await dans main

**Fichier:** `os_fs/src/main.fsh` (à la fin)

```flux
public async void main_async(long boot_addr, long magic) => {
    uint data_task = await async_read_file("/boot/config");
    long result = async_get_result(data_task);
}
```

---

## 🐛 Troubleshooting

### Erreur: "fluxc: command not found"

```bash
# Compiler le compilateur Flux d'abord
cd /mnt/linux_plus/projet_dev/flux_grid_os
cargo build -p fluxc --target x86_64-unknown-linux-gnu

# Ensuite compiler les fichiers Flux
./build_flux_multi.sh
```

### Erreur: "Syntax Error"

1. Vérifier la ligne mentionnée
2. Consulter `FLUX_LANGUAGE_SUMMARY.md` pour la syntaxe correcte
3. Comparer avec les exemples dans `async_example.fsh` ou `test.fsh`

### Erreur: "Unresolved symbol"

Vérifier que:
- La fonction est déclarée dans lib.fsh ou main.fsh
- Le nom est exactement identique (case-sensitive)
- La signature de la fonction correspond

---

## 💡 Tips & Tricks

### Générer l'Assembleur

```bash
fluxc compile -S os_fs/src/main.fsh -o kernel.asm
```

### Compiler Avec Symboles Debug

```bash
./build_flux_multi.sh -g
```

### Vérifier la Grammaire

```bash
# Compiler le fichier de test
fluxc compile os_fs/src/test.fsh
```

### Voir les Performances Async

```flux
public void check_performance() => {
    PerformanceMetrics metrics = async_get_metrics();
    // metrics.completed_tasks
    // metrics.failed_tasks
    // metrics.total_execution_time
}
```

---

## 📚 Ressources

### Fichiers à Consulter
```
fluxc/src/flux_grammar.pest              # Grammaire PEST
os_fs/src/main.fsh                       # Point d'entrée
os_fs/src/lib.fsh                        # Noyau + async
os_fs/src/async_example.fsh              # Examples
os_fs/src/test.fsh                       # Tests
```

### Guides à Lire
```
guides/FLUX_LANGUAGE_SUMMARY.md          # Démarrer ici
guides/ASYNC_AWAIT_GUIDE.md              # Async complet
guides/COMPILATION_SYSTEM.md             # Build system
guides/IMPLEMENTATION_COMPLETE.md        # Synthèse
guides/FILES_MANIFEST.md                 # Tous les fichiers
```

---

## ✅ Checklist de Démarrage

- [ ] Compiler avec `./build_flux_multi.sh`
- [ ] Vérifier `os_fs/build/kernel.bin` est créé
- [ ] Lire `guides/FLUX_LANGUAGE_SUMMARY.md`
- [ ] Regarder `os_fs/src/async_example.fsh`
- [ ] Créer ma première fonction Flux
- [ ] Compiler mes modifications
- [ ] Lire le guide async/await complet
- [ ] Intégrer avec le build Rust existant

---

## 🎓 Concepts Clés à Retenir

### 1. **Async/Await Optimisé**
- Tasks: 40 bytes chacune
- Queue circulaire: O(1) operations
- Event loop: non-blocking
- Performance: ~86 KB pour tout le système

### 2. **Compilation Multi-Fichiers**
- Linking automatique
- Résolution des symboles
- Support complet des appels cross-file

### 3. **Grammaire Complète**
- 45+ règles PEG
- Support hex/binary/octal
- If/else, loops, async/await
- Appels de fonction avancés

### 4. **Structure du Projet**
```
main.fsh    → Point d'entrée _start
lib.fsh     → Noyau + async runtime
main.rs     → Rust wrapper
```

---

## 🚀 Prochaines Actions

### Immediate (Aujourd'hui)
```bash
1. ./build_flux_multi.sh                  # Compiler
2. cat guides/FLUX_LANGUAGE_SUMMARY.md    # Lire overview
3. cat os_fs/src/async_example.fsh        # Voir examples
```

### This Week
```bash
1. Créer ma première fonction async
2. Intégrer avec le build Rust
3. Tester la compilation complète
```

### This Month
```bash
1. Implémenter des drivers async
2. Ajouter GPU integration
3. Optimiser les performances
```

---

## 🤝 Aide Supplémentaire

### Syntaxe Flux?
➜ `guides/FLUX_LANGUAGE_SUMMARY.md`

### Async/Await?
➜ `guides/ASYNC_AWAIT_GUIDE.md`

### Build System?
➜ `guides/COMPILATION_SYSTEM.md`

### Tous les fichiers?
➜ `guides/FILES_MANIFEST.md`

### Vue d'ensemble?
➜ `guides/IMPLEMENTATION_COMPLETE.md`

---

## 🎉 Ready?

```bash
./build_flux_multi.sh && echo "✨ Success!"
```

Let's build FluxGridOS! 🚀

