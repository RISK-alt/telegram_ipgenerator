# Générateur d'IP avec Test Réseau et Log de Vitesse

⚠️ **ACCÈS RESTREINT** ⚠️
Ce projet est propriétaire et son accès est strictement limité. Toute utilisation non autorisée est interdite.

Un générateur d'adresses IP performant qui teste la validité des IPs générées et les envoie automatiquement sur Telegram par lots.

## ⚠️ Avertissement Légal

Ce logiciel est protégé par une licence propriétaire. Toute utilisation, modification, distribution ou reproduction sans autorisation explicite est strictement interdite et fera l'objet de poursuites judiciaires.

## Sécurité du Repository

Ce repository est privé et protégé. Les mesures de sécurité suivantes sont en place :

1. **Accès Restreint**
   - Le repository est privé
   - L'accès en lecture est limité aux collaborateurs autorisés
   - L'authentification à deux facteurs (2FA) est obligatoire
   - Les tokens d'accès personnels (PAT) sont requis

2. **Protection contre le Clonage Non Autorisé**
   - Surveillance active des tentatives d'accès
   - Détection des tentatives de clonage non autorisées
   - Blocage automatique des adresses IP suspectes
   - Journalisation de toutes les tentatives d'accès

3. **Sécurité du Code**
   - Chiffrement du code source
   - Protection contre la copie non autorisée
   - Détection des fuites de code
   - Watermarking numérique

## Fonctionnalités

- Génération continue d'adresses IP aléatoires
- Test de validité des IPs via connexion TCP
- Capacité de génération : 1000+ IPs valides par minute
- Envoi automatique sur Telegram par lots de 50 000 IPs
- Système anti-doublons
- Monitoring en temps réel des performances
- Architecture asynchrone et parallèle

## Accès et Autorisation

L'accès à ce projet est restreint. Pour obtenir l'autorisation d'utilisation :

1. Contactez ElevateOps via les canaux officiels
2. Fournissez les informations requises pour la vérification
3. Attendez l'approbation écrite
4. Recevez les instructions d'accès sécurisé
5. Respectez les conditions d'utilisation spécifiées

## Prérequis

- Rust (dernière version stable)
- Un bot Telegram et son token
- Un ID de chat Telegram valide
- Autorisation d'utilisation

## Installation

⚠️ **IMPORTANT** : Ce repository est privé et protégé. L'installation n'est possible qu'après autorisation.

1. Obtenez l'autorisation d'accès
2. Configurez votre authentification GitHub avec 2FA
3. Générez un token d'accès personnel (PAT)
4. Clonez le repository avec authentification :
```bash
git clone https://[VOTRE_PAT]@github.com/[ORGANISATION]/telegram_ipgenerator.git
cd telegram_ipgenerator
```

2. Configurez les variables d'environnement dans `main.rs` :
```rust
const TELEGRAM_BOT_TOKEN: &str = "VOTRE_TOKEN_BOT";
const TELEGRAM_CHAT_ID: &str = "VOTRE_CHAT_ID";
```

3. Installez les dépendances :
```bash
cargo build --release
```

## Utilisation

Lancez le programme :
```bash
cargo run --release
```

Le programme va :
1. Générer des IPs aléatoires
2. Tester leur validité
3. Les stocker par lots de 50 000
4. Envoyer automatiquement les lots sur Telegram
5. Afficher les statistiques toutes les 10 secondes

## Configuration

Vous pouvez ajuster les paramètres suivants dans le code :

- `BATCH_SIZE` : Taille des lots d'IPs (défaut : 50 000)
- `WORKER_COUNT` : Nombre de workers parallèles (défaut : 8)
- `TCP_TIMEOUT_MS` : Timeout pour les tests TCP (défaut : 200ms)

## Structure des fichiers

- `main.rs` : Code principal
- `sent_ips.txt` : Historique des IPs déjà envoyées
- `ips_batch.txt` : Fichier temporaire pour les lots d'IPs

## Monitoring

Le programme affiche des statistiques toutes les 10 secondes :
- Nombre total d'IPs générées
- Nombre d'IPs valides envoyées
- Vitesse de génération (IPs/min)

## Sécurité

- Les IPs sont testées avant d'être considérées comme valides
- Système anti-doublons intégré
- Pas de stockage permanent des IPs invalides
- Protection contre l'utilisation non autorisée

## Licence

Ce projet est sous licence propriétaire. Tous droits réservés.
Voir le fichier `LICENSE` pour plus de détails.

## Contact

Pour toute demande d'autorisation ou d'information :
- Email : [VOTRE_EMAIL]
- Telegram : [VOTRE_TELEGRAM]
- Site web : [VOTRE_SITE]

⚠️ Ne partagez jamais vos tokens d'accès ou informations d'authentification.

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
1. Fork le projet
2. Créer une branche pour votre fonctionnalité
3. Commiter vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request 