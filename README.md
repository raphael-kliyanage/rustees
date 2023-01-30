# rustees

### Pour lancer les programmes
Lancer 2 terminal et saisir les commandes séparemment :
```
cargo run --bin serveur
cargo run --bin client
```

### Pour implémenter notre code non-validé
Poussez votre code dans la branche "dev" 
```
git pull
git add *
git commit -m "msg"
git status => regardez si vous êtes bien dans la branche dev
git pull
git push
```

Faites un merge vers la branche "pre-prod"
```
git checkout pre-prod
git pull
git merge dev
git checkout dev
```

Notifiez le groupe Teams pour valider

### La branche main est réservé uniquement lorsque tous les membres ont validés le code
La branche "dev" est notre branche réservée pour ajouter les nouvelles fonctionnalitées.
La branche "pre-prod" correspond à nos release sous validation.