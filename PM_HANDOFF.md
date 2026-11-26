# PM_HANDOFF.md — fichier du chef de projet

Écrit UNIQUEMENT par la conversation « chef de projet » (Claude, app de bureau).
Claude Code ne le modifie jamais. Chaque nouvelle conversation chef de projet
commence par lire ce fichier en entier, puis les dernières entrées de `SESSIONS.md`.

## Objectif du projet
Rendre le monde du jeu jouable sur mon PC avec la couche de compatibilité
(fork de VIctory). Situation au départ (README, 2026-11-25) : 8 à 10 fps dans
les premières pièces, zones d'ombre noires (le ray tracing ne tourne pas encore),
environ 24 Go de mémoire utilisés.

## Ma machine
- Windows : 11 Pro | CPU : AMD Ryzen 7 3700X (8 cœurs, AVX2) | GPU : NVIDIA RTX 3060 (VRAM 12 Go, pilote 32.0.16.2124) | RAM : 16 Go
- Ma copie du jeu : dans un dossier hors du dépôt, réglée dans `victory.toml`

## Dépôt
- Mon fork : https://github.com/lahouuuu/victory-fork
- Remotes : `origin` = mon fork, `upstream` = fxceless/victory-fork
  (le remote `victory` = Marvisak/VIctory n'est PAS configuré, constaté le 2026-11-26 ; pas nécessaire pour l'instant)
- Branche de travail actuelle : `main` (décision : on travaille directement sur main de mon fork, une session = un ou plusieurs commits, je pousse après vérification)
- Dernier commit validé et poussé : `085ee7c` (= origin/main, état du fork au 2026-11-25)
- Au 2026-11-26 : CLAUDE.md, SESSIONS.md, PROMPTS.md, PM_HANDOFF.md ne sont pas encore suivis par git ; la Session 1 doit les committer

## Mes demandes et priorités (dans l'ordre)
1. Session 1 : installation + tests verts + mesure de référence sur ma machine
2. Session 2 : fusionner les 47 commits d'upstream (décidé 2026-11-26 : nécessaires, voir ci-dessous)
3. Ensuite : que le jeu tourne « le mieux possible » sur ma machine (stabilité d'abord, puis fps). Ma consigne : ne pas perdre de temps sur ce qui ne rend pas le jeu plus stable / plus jouable.

## Décisions prises
- 2026-11-26 : travail sur `main`, Claude Code committe sans pousser, je pousse après vérification du chef de projet.
- 2026-11-26 : en pratique Claude Code lance lui-même le jeu pour les mesures et relève les chiffres ; je joue (cinématique + première pièce). Accepté.
- 2026-11-26 : objectif 30 fps = hors de portée à court terme (README : ~20 fps max même sans surcoût VIctory sur une RTX 4070). Objectif réaliste d'abord : que le jeu tourne sans planter sur ma machine.

## Sessions Claude Code
| Session | Tâche | Résultat | Commit | Suite |
|---------|-------|----------|--------|-------|
| 1 | Installation et mesure de référence | en cours : 2 plantages (RAM libre 0,2–0,4 Go), 1er passage cinématique 1–2 fps (compilation shaders). Attente : fichier d'échange + fermeture des applis, puis relance mesurée | départ 085ee7c | |

## Problèmes ouverts / idées pour plus tard
- RISQUE MÉMOIRE : le README mesure ~24 Go utilisés (~16 Go en RAM) sur un PC de test à 32 Go. Ma machine n'a que 16 Go de RAM → risque d'erreur Windows 1455 / plantage ; il faudra sans doute agrandir le fichier d'échange. Session 1 doit le vérifier.
- Machine de test du README : i7-14700KF + RTX 4070. Ma machine (Ryzen 7 3700X + RTX 3060) est plus lente : s'attendre à moins que 8–10 fps.
- Ombres noires = ray tracing pas encore branché (docs/fork-ray-tracing.md).
- upstream (fxceless) a 47 commits que nous n'avons pas (vérifié le 2026-11-26) : meilleur jeu de switches (« run 279 »), corrections de stabilité, plus loin dans le jeu selon Claude Code. Vérifié par le PM dans upstream/README : correction de l'eau (`VICTORY_TEXTURE_BACK=1`, sans elle les bateaux coulent et la mission échoue = blocage de progression), jeu de switches run 279 (9–12 fps), outils pour diagnostiquer les pertes de carte après 30–50 min. → DÉCIDÉ : Session 2 = fusion. Attention : upstream utilise 23–27 Go de RAM sur 32 Go, pire pour moi (16 Go). Le diff fait +333 000 lignes, surtout des journaux de diagnostic « scrubbed » : faire tourner `cargo xtask policy` et vérifier qu'aucun ne contient de données du jeu.
- Mémoire Session 1 : le jeu engage ~16,5 Go, VRAM 6,2 Go ; système 44,5 / 56,9 Go engagés → la limite d'engagement n'était PAS atteinte ; le manque est la RAM physique (0,4 Go libre). Fermer les autres applis compte plus que le fichier d'échange.

## Pour la prochaine conversation chef de projet
- Attendre le « done » de la Session 1, vérifier l'entrée SESSIONS.md + le diff (aucune donnée du jeu, aucun chemin perso vers la copie du jeu dans le dépôt), puis choisir la Session 2 selon la mesure de référence.
