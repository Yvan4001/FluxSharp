# 🏛️ Classes dans FluxGridOS Flux Language

## Vue d'Ensemble

Les **classes** dans Flux offrent une programmation orientée objet complète avec:
- ✅ Propriétés publiques/privées
- ✅ Méthodes publiques/privées
- ✅ Constructeurs
- ✅ Héritage simple
- ✅ Support async/await
- ✅ Attributs d'alignement

---

## Syntaxe de Base

### Définition Simple

```flux
public class MyClass {
    public uint value;
    public string name;
    
    public MyClass(uint v, string n) => {
        value = v;
        name = n;
    }
    
    public void printValue() => {
        serial_print("Value: ");
    }
}
```

### Utilisation

```flux
MyClass obj;  // Créer une instance
obj = MyClass(42, "test");  // Initialiser
obj.printValue();  // Appeler une méthode
```

---

## Syntaxe Complète

### Classe Avec Toutes les Fonctionnalités

```flux
[Align(4096)]  // Optionnel: attribut d'alignement
public class ClassName : BaseClass {  // Optionnel: héritage
    
    // Propriétés publiques
    public type property1;
    public type property2;
    
    // Propriétés privées
    private type internalState;
    private bool isInitialized;
    
    // Constructeur
    public ClassName(type arg1, type arg2) => {
        property1 = arg1;
        internalState = arg2;
        isInitialized = true;
    }
    
    // Méthode publique
    public type methodName() => {
        return property1;
    }
    
    // Méthode async publique
    public async void asyncMethod() => {
        await someAsyncTask();
    }
    
    // Méthode privée
    private void internalMethod() => {
        internalState = 0;
    }
}
```

---

## Modificateurs de Visibilité

### Public

```flux
public class MyClass {
    public uint value;  // Accessible partout
    
    public void publicMethod() => {  // Accessible partout
    }
}
```

### Private

```flux
public class MyClass {
    private uint secret;  // Seulement dans la classe
    
    private void privateMethod() => {  // Seulement dans la classe
    }
}
```

### Par Défaut (sans modificateur)

```flux
public class MyClass {
    uint implicitPublic;  // Publique par défaut
    
    void implicitPublic() => {  // Publique par défaut
    }
}
```

---

## Constructeurs

### Syntaxe

```flux
public class Point {
    public long x;
    public long y;
    
    // Constructeur avec même nom que la classe
    public Point(long px, long py) => {
        x = px;
        y = py;
    }
}
```

### Sans Arguments

```flux
public class Default {
    public uint count;
    
    public Default() => {
        count = 0;
    }
}
```

### Avec Initialisation Complexe

```flux
public class Complex {
    private long[256] buffer;
    private uint capacity;
    
    public Complex(uint cap) => {
        capacity = cap;
        for(uint i = 0; i < capacity; i++) {
            buffer[i] = 0;
        }
    }
}
```

---

## Méthodes

### Méthode Simple

```flux
public void greet() => {
    serial_print("Hello!");
}
```

### Avec Paramètres

```flux
public void add(uint a, uint b) => {
    uint result = a + b;
}
```

### Avec Return Value

```flux
public uint getValue() => {
    return value;
}

public bool isValid() => {
    return valid == true;
}
```

### Méthode Async

```flux
public async void fetchData() => {
    uint task = await async_read_file("/boot/data");
}

public async long processAsync() => {
    long result = await someAsyncTask();
    return result;
}
```

### Méthode Privée

```flux
private void internalHelper() => {
    // Logique interne
}

private void updateState() => {
    isValid = false;
}
```

---

## Héritage

### Classe de Base

```flux
public class Animal {
    public string name;
    
    public Animal(string n) => {
        name = n;
    }
    
    public void speak() => {
        serial_print(name);
    }
}
```

### Classe Dérivée

```flux
public class Dog : Animal {
    public string breed;
    
    public Dog(string n, string b) => {
        breed = b;
    }
    
    public void bark() => {
        serial_print("Woof!");
    }
}
```

### Utilisation

```flux
Animal pet;
Dog dog;

pet = Animal("Generic");
dog = Dog("Rex", "Labrador");

pet.speak();  // Appelle Animal.speak()
dog.speak();  // Hérité d'Animal
dog.bark();   // Spécifique à Dog
```

---

## Types Personnalisés comme Champs

### Struct dans Classe

```flux
public struct Point {
    uint x;
    uint y;
}

public class Shape {
    public Point position;
    public uint color;
    
    public Shape() => {
        color = 0xFF0000;
    }
}
```

### Classe dans Classe

```flux
public class Engine {
    public uint rpm;
}

public class Car {
    public Engine engine;
    public string model;
    
    public Car() => {
        model = "Unknown";
    }
}
```

### Arrays de Types Personnalisés

```flux
public class GameWorld {
    private Point[1024] entities;
    private uint entityCount;
    
    public void addEntity(Point p) => {
        entities[entityCount] = p;
        entityCount++;
    }
}
```

---

## Attributs

### Alignement

```flux
[Align(4096)]
public class PageAlignedData {
    public uint[1024] data;
}

[Align(64)]
public class CacheAligned {
    public long values[8];
}
```

### Packed (pas de padding)

```flux
[Packed]
public class CompactData {
    public byte flags;
    public uint value;
}
```

---

## Exemples Pratiques

### Example 1: Task Manager

```flux
public class Task {
    public uint id;
    public uint status;
    public long priority;
    
    public Task(uint taskId) => {
        id = taskId;
        status = 0;
        priority = 0;
    }
    
    public async void execute() => {
        status = 1;
        await async_sleep(1000);
        status = 2;
    }
}

public class TaskQueue {
    private Task[64] tasks;
    private uint count;
    
    public uint addTask(uint taskId) => {
        Task t;
        t = Task(taskId);
        tasks[count] = t;
        count++;
        return taskId;
    }
}
```

### Example 2: Memory Manager

```flux
public class MemoryBlock {
    private long[256] data;
    private bool allocated;
    
    public MemoryBlock() => {
        allocated = false;
    }
    
    public void allocate() => {
        allocated = true;
    }
    
    public void deallocate() => {
        allocated = false;
    }
    
    public bool isAllocated() => {
        return allocated;
    }
}

public class MemoryManager {
    private MemoryBlock[32] blocks;
    
    public MemoryManager() => {
    }
    
    public MemoryBlock allocateBlock() => {
        MemoryBlock b;
        b = MemoryBlock();
        b.allocate();
        return b;
    }
}
```

### Example 3: Configuration System

```flux
public class Setting {
    public string key;
    public string value;
    
    public Setting(string k, string v) => {
        key = k;
        value = v;
    }
}

public class AppConfig {
    private Setting[32] settings;
    private uint settingCount;
    
    public void setSetting(string key, string value) => {
        Setting s;
        s = Setting(key, value);
        settings[settingCount] = s;
        settingCount++;
    }
    
    public string getSetting(string key) => {
        string result = "";
        return result;
    }
}
```

---

## Comparaison: Struct vs Class

| Feature | Struct | Class |
|---------|--------|-------|
| Méthodes | Non | Oui |
| Constructeur | Non | Oui |
| Héritage | Non | Oui |
| Visibilité (public/private) | Non | Oui |
| Async methods | Non | Oui |
| Use case | Données simples | Logique complexe |

---

## Avantages des Classes

### Encapsulation
```flux
public class SecureCounter {
    private uint count;  // Protégé
    
    public void increment() => {
        count++;
    }
    
    public uint getCount() => {
        return count;
    }
}
```

### Réutilisabilité via Héritage
```flux
public class Vehicle {
    public uint speed;
    
    public void accelerate() => {
        speed++;
    }
}

public class Car : Vehicle {
    public void honk() => {
    }
}
```

### Encapsulation du Comportement
```flux
public class Logger {
    private string[256] logs;
    private uint logCount;
    
    public void log(string message) => {
        logs[logCount] = message;
        logCount++;
    }
    
    public void clear() => {
        logCount = 0;
    }
}
```

---

## Bonnes Pratiques

### ✅ À Faire

```flux
// Classes bien structurées
public class GoodClass {
    // Propriétés privées
    private uint internalState;
    
    // Propriétés publiques
    public string name;
    
    // Constructeur explicite
    public GoodClass(string n) => {
        name = n;
        internalState = 0;
    }
    
    // Méthodes privées pour logique interne
    private void updateInternal() => {
        internalState++;
    }
    
    // Méthodes publiques pour interface
    public void publicAction() => {
        updateInternal();
    }
}
```

### ❌ À Éviter

```flux
// Classes mal structurées
public class BadClass {
    public uint[256] internalBuffer;  // Expose impl
    
    // Pas de constructeur
    
    public void doSomething() => {
        // Logique complexe sans abstraction
    }
}
```

---

## Intégration avec Async/Await

```flux
public class AsyncProcessor {
    public async void processData() => {
        uint taskId = await async_create_task();
        async_schedule_task(taskId);
        
        long result = async_await_task(taskId);
    }
    
    public async void parallelProcess() => {
        async_parallel();
    }
}
```

---

## Fichier Exemple Complet

Voir `os_fs/src/class_examples.fsh` pour des exemples complets de classes.

---

## Compilation

Les classes compilent comme toutes les autres constructions Flux:

```bash
cargo run -p fluxc --target x86_64-unknown-linux-gnu -- compile --all os_fs/src -o kernel.bin

# Ou avec le script
./build_complete.sh
```

---

## Limitatiions Actuelles

⚠️ **À noter:**
- Pas de méthodes de classe (static methods)
- Pas d'interfaces/traits
- Pas de generics
- Pas d'opérateurs surchargés

Ces features pourront être ajoutées dans les futures versions!

---

**Classes Flux: Programmation Orientée Objet pour Noyaux OS! 🏛️**

