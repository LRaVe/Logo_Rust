# logo 

## Fonction de chaque fichier


- src/main.rs: pipeline principal du projet (lecture du programme Logo, lexing, parsing, construction AST, evaluation, puis generation du SVG).
- src/lexer.rs: definition des tokens et regles lexicales pour transformer le texte Logo en lexemes.
- src/parser.rs: definition de la grammaire Logo et construction des arbres de parsing.
- src/ast.rs: conversion parse tree vers AST metier et evaluation de cet AST.
- src/svg_compiler.rs: compilation de l AST en primitives SVG (segments, orientation, etat du stylo).
- src/lib.rs: declaration des modules partageables de la crate.
- src/bin/svg_demo.rs: binaire de demonstration independant qui genere un carre SVG sans passer par le langage Logo complet.
