// Test de syntaxe float et double avec Flux#
// Les floats se terminent par 'f' ou 'F'
// Les doubles utilisent la notation avec point décimal et optionnellement la notation scientifique

public static void Main() {
    // Déclaration de floats avec suffixe 'f'
    float pi = 3.14159f;
    float e = 2.71828f;
    float golden_ratio = 1.618034f;
    
    // Déclaration de doubles avec notation décimale
    double sqrt2 = 1.41421356;
    double sqrt3 = 1.73205080;
    double ln10 = 2.302585093;
    
    // Opérations arithmétiques sur floats
    float sum_floats = 1.5f + 2.5f;
    float product_floats = 3.0f * 2.0f;
    float quotient_floats = 10.0f / 2.0f;
    
    // Opérations arithmétiques sur doubles
    double sum_doubles = 1.5 + 2.5;
    double product_doubles = 3.0 * 2.0;
    double quotient_doubles = 10.0 / 2.0;
    
    // Affichage des valeurs
    print(pi);
    print(e);
    print(golden_ratio);
    
    print(sqrt2);
    print(sqrt3);
    print(ln10);
    
    print(sum_floats);
    print(product_floats);
    print(quotient_floats);
    
    print(sum_doubles);
    print(product_doubles);
    print(quotient_doubles);
}

