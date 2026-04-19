# Projet nat_set

Structures de données pour ensembles d'entiers en Rust, dans le cadre du Bureau d'Études (BE) de Licence Informatique 2025-2026, sous la direction de Millian Poquet.

**Objectif** : implémenter et comparer plusieurs représentations d'ensembles de ressources (`u32`) -- liste triée, `HashSet`, ensembles d'intervalles, Roaring Bitmaps -- en respectant un trait commun `ResourceSet`, et évaluer leurs performances sur les opérations ensemblistes (union, intersection, différence) et la dé/sérialisation.

**État** : quatre structures fonctionnelles, validées par tests unitaires et property‑based testing (`proptest`). Les benchmarks préliminaires montrent que `RoaringSet` est globalement le plus performant, mais `IntervalSet` excelle pour les tests d'appartenance sur ensembles denses. Une piste avancée (`SzemerSet`) a été explorée mais s'est révélée trop complexe pour le temps imparti.

**Rapports** :

- [Rapport prévisionnel (CC1)](CC1.md) -- méthode et planification  
- [Rapport de mi‑parcours (CC2)](CC2.md) -- implémentation, benchmarks, difficultés