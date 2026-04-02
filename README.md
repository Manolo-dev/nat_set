---
geometry: "left=2cm,right=2cm,top=2cm,bottom=2cm"
header-includes: |
    \usepackage{stmaryrd}
---

# CC1 -- Phase individuelle : méthode et planification

## Approche technique

- **Méthodologie.** Le développement suivra un cycle itératif par implémentation, inspiré du TDD : pour chaque structure, on écrit les tests la concernant, on implémente jusqu'à leur satisfaction, puis on évalue les performances avant de passer à la suivante. Le trait commun est défini en amont et affiné à chaque cycle si nécessaire. La suite de tests s'accumule ainsi de cycle en cycle, garantissant la non-régression des implémentations précédentes.

- **Trait Rust.** L'API sera définie en premier, couvrant : insertion/suppression, appartenance, itération ordonnée, opérations ensemblistes (union, intersection, différence, différence symétrique), sérialisation/désérialisation. Le trait sera conçu pour être minimalement contraint afin de ne pas pénaliser structurellement certaines représentations.

- **Implémentations.** Les structures imposées, dans l'ordre croissant de complexité : `HashSet` stdlib (baseline), bitset dense, liste triée d'intervalles contigus, vecteur dynamique d'intervalles compressé, arbre AVL (maison), arbre rouge-noir (maison). La veille bibliographique (conduite en parallèle dès S1) orientera le choix de structures additionnelles ; pistes identifiées à ce stade : *Roaring Bitmaps* (bitset hybride creux/dense), ensembles de *Szemerédi* (ensembles d'intervalles à pas ; généralisation directe des listes d'intervalles contigus où chaque intervalle est une progression arithmétique (début, fin, pas), la structure représentant leur union -- le nom fait référence au théorème de Szemerédi sur les progressions arithmétiques dans les ensembles denses d'entiers), *skip list* ordonnée.

- **Validation fonctionnelle.** Chaque implémentation sera soumise à une suite de tests unitaires partagée : propriétés algébriques (associativité, commutativité, lois de De Morgan, idempotence), cas limites (ensemble vide, singleton, ensemble plein, chevauchement d'intervalles). `proptest` sera utilisé pour le property-based testing.

- **Évaluation des performances.** Benchmarks via `criterion.rs`. Paramètres variés : taille du domaine $n$ ($S \subseteq \llbracket 0, n - 1 \rrbracket$), densité $\delta = \frac{|S|}n$ et fragmentation $\phi = \frac{|\{i \in S | i - 1 \not\in S \}|}{|S|}$ de l'ensemble, taille des opérandes. La dé/sérialisation sera benchmarkée séparément. Les expériences tourneront en monocœur avec isolation CPU (`taskset`, fréquence fixée) pour la reproductibilité.

- **Formation.** Deux blocs dédiés : S1 pour les fondamentaux Rust (*The Book*, `criterion.rs`, `proptest`) ; S5 pour l'unsafe Rust (*Rustonomicon*) si requis par des implémentations bas-niveau (SIMD éventuel).

## Planification (phase 1, jusqu'au 20 avril)

| Semaine | Objectif |
|-|-|
| S1–S2   | **Formation** : traits, generics, lifetimes, `cargo` |
| | Définition du trait |
| | `HashSet` + bitset (baseline) |
| | **Début veille biblio** |
| S3–S4   | Intervalles contigus + ensembles de Szemerédi |
| | Suite de tests complète |
| S5–S6   | AVL + rouge-noir |
| | Structure(s) issue(s) de la veille |
| | **Formation** : unsafe Rust si nécessaire |
| S7      | Protocole expérimental complet |
| | Collecte et analyse des résultats |
| S8      | Rédaction du mini-article (Typst/LaTeX) |