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
- Dernier commit validé : `57e685d` (Session 1, vérifié par le PM le 2026-11-26 ; à pousser par moi : `git push origin main`). Avant : `085ee7c`
- CLAUDE.md, SESSIONS.md, PROMPTS.md, PM_HANDOFF.md sont suivis par git depuis 57e685d. Mes modifs de PM_HANDOFF.md après ce commit restent non committées : la session suivante les inclut dans son commit.

## Mes demandes et priorités (dans l'ordre)
1. Session 1 : installation + tests verts + mesure de référence sur ma machine
2. Session 2 : fusionner les 47 commits d'upstream (décidé 2026-11-26 : nécessaires, voir ci-dessous)
3. Ensuite : que le jeu tourne « le mieux possible » sur ma machine (stabilité d'abord, puis fps). Ma consigne : ne pas perdre de temps sur ce qui ne rend pas le jeu plus stable / plus jouable.

## Décisions prises
- 2026-11-26 : on n'importe JAMAIS diagnostics/ d'upstream (journaux avec noms de fichiers du jeu + identifiant Xbox personnel). Upstream est intégré par commit « squash » (pas de merge) pour que ces journaux n'entrent pas dans notre historique. Fusions suivantes : appliquer seulement `git diff <dernier commit upstream intégré> upstream/main -- . ':!diagnostics'`. Dernier commit upstream intégré : 988aa41 (66 commits ; upstream a avancé pendant la session, au lieu de fc63ce1/47 prévus), une fois la Session 2 terminée. NB : upstream a aussi changé docs/asset-policy.md (section « Run logs », décision du propriétaire d’upstream) et xtask/policy.rs pour autoriser diagnostics/ : chez nous ce dossier reste exclu ; ajouter plus tard une protection (gitignore ou policy) contre son retour.
- 2026-11-26 : fmt et clippy échouent DÉJÀ sur le code de départ (1 476 écarts fmt, 8/10 erreurs clippy dans victory-gpu) et 2 tests WARP sont instables. Règle : une session ne doit pas AJOUTER d'échec par rapport à la base ; on ne corrige pas ces échecs tant que ça n'aide pas le jeu (ma consigne).
- 2026-11-26 : le fichier d'échange n'est PAS en cause (limite 56,9 Go jamais approchée). Avant chaque mesure : redémarrer et fermer les autres applis (ça a évité le plantage au lancement 3).
- 2026-11-26 : travail sur `main`, Claude Code committe sans pousser, je pousse après vérification du chef de projet.
- 2026-11-26 : en pratique Claude Code lance lui-même le jeu pour les mesures et relève les chiffres ; je joue (cinématique + première pièce). Accepté.
- 2026-11-26 : objectif 30 fps = hors de portée à court terme (README : ~20 fps max même sans surcoût VIctory sur une RTX 4070). Objectif réaliste d'abord : que le jeu tourne sans planter sur ma machine.

## Sessions Claude Code
| Session | Tâche | Résultat | Commit | Suite |
|---------|-------|----------|--------|-------|
| 1 | Installation et mesure de référence | OK (vérifié). Lancement 3 (redémarré, applis fermées) : cinématique 4,4 fps moy., 1re pièce ~1 fps (1re visite, shaders en compilation), pas de plantage. Lancements 1-2 : plantage 0xc0000005 dans le code du jeu, RAM libre ~0. Pic : 23 Go engagés par le jeu, RAM libre 0,1 Go, VRAM 9,4 Go | 085ee7c → 57e685d | Session 2 : fusion upstream + remesure |
| 2 | Fusion des 47 commits d'upstream + mesure 1re pièce cache chaud (switches 253 et 279) | en cours. 1er essai : merge 542c10c (local, jamais poussé) ANNULÉ : les journaux diagnostics/ d'upstream contiennent des noms de fichiers internes du jeu (« file opened path=… ») et le XUID Xbox du testeur upstream. Décision PM : option 1 = reset --keep 57e685d + squash d’upstream SANS diagnostics/. Squash bc5db4c vérifié par le PM (1 seul parent, arbre = upstream 988aa41 sans diagnostics/ + nos 4 .md). 57e685d poussé. Reste : vérifications, build, mesures A/B | départ 57e685d | |

## Problèmes ouverts / idées pour plus tard
- RISQUE MÉMOIRE : le README mesure ~24 Go utilisés (~16 Go en RAM) sur un PC de test à 32 Go. Ma machine n'a que 16 Go de RAM → risque d'erreur Windows 1455 / plantage ; il faudra sans doute agrandir le fichier d'échange. Session 1 doit le vérifier.
- Machine de test du README : i7-14700KF + RTX 4070. Ma machine (Ryzen 7 3700X + RTX 3060) est plus lente : s'attendre à moins que 8–10 fps.
- Ombres noires = ray tracing pas encore branché (docs/fork-ray-tracing.md).
- upstream (fxceless) a 47 commits que nous n'avons pas (vérifié le 2026-11-26) : meilleur jeu de switches (« run 279 »), corrections de stabilité, plus loin dans le jeu selon Claude Code. Vérifié par le PM dans upstream/README : correction de l'eau (`VICTORY_TEXTURE_BACK=1`, sans elle les bateaux coulent et la mission échoue = blocage de progression), jeu de switches run 279 (9–12 fps), outils pour diagnostiquer les pertes de carte après 30–50 min. → DÉCIDÉ : Session 2 = fusion. Attention : upstream utilise 23–27 Go de RAM sur 32 Go, pire pour moi (16 Go). Le diff fait +333 000 lignes, surtout des journaux de diagnostic « scrubbed » : faire tourner `cargo xtask policy` et vérifier qu'aucun ne contient de données du jeu.
- Mémoire Session 1 : le jeu engage ~16,5 Go, VRAM 6,2 Go ; système 44,5 / 56,9 Go engagés → la limite d'engagement n'était PAS atteinte ; le manque est la RAM physique (0,4 Go libre). Fermer les autres applis compte plus que le fichier d'échange.

- 2026-11-26 midi : je signale que d'autres projets git disent atteindre 30 fps. PM : intéressant mais à vérifier, liens demandés. Conditions avant d'en reprendre quoi que ce soit : (1) licence compatible avec la nôtre (GPL), (2) chiffre vérifiable (quelle scène, quel PC, quels switches ; le README d'upstream dit ~20 fps max même sans surcoût VIctory sur RTX 4070, donc « 30 fps en jeu » est suspect), (3) Claude Code lit le code avant de le compiler (sécurité), (4) règle n°1 vérifiée, (5) reprise ciblée (cherry-pick / squash), une amélioration par session, derrière un switch, mesurée chez moi. Après la fin de la Session 2.

- 2026-11-26 12:40 : post Reddit (r/FitGirlRepack, « 30 fps RTX 5090 ») vu par moi. Vérifié par le PM via l'API GitHub : la vraie source est l'issue fxceless/victory-fork#1268 (RTX 4080 : 27 fps médian, profil `--console series-s`, rendu interne 720p `VICTORY_RENDER_SCALE=0.6666667` + `VICTORY_OWN_UPSCALER=1`, ray tracing coupé, carte GPU à 99 %). Base : trustno1-coder/VIctory-NVIDIA (GPL-3.0, « No game files », dérivé d'avatarmanz/VIctory-7900XTX, GPL-3.0) + PR de fxceless et PastiiJay/klbf-lab (GPL-3.0). La plupart de ces switches N'EXISTENT PAS dans notre arbre (RENDER_SCALE, OWN_UPSCALER, NAN_SCRUB… absents ; seul le profil series-s existe déjà). Chez moi (RTX 3060 ≈ 40 % d'une 4080, 12 Go VRAM alors que la recette monte à 12,8 Go) : espérer plutôt 12–15 fps, pas 30. PLAN : Session 2 + mesure C (profil series-s, faisable tout de suite) ; Session 3 = branche d'essai partant de trustno1-coder/VIctory-NVIDIA, audit règle n°1 + lecture des scripts, build depuis les sources uniquement (jamais d'exe téléchargé), mesure avec la recette adaptée à 12 Go. Si nettement meilleur → on change de base. Pièges cités : `VICTORY_THIN=1` (floute sur RTX 50 seulement), `SKIP_WC_*` (monde noir chez certains), `VICTORY_PIPELINE_WAIT_MS` 10000 → gels de 12 s (mettre 150).

- 2026-11-26 13:00 : recherche PM des dépôts les plus avancés (API GitHub, README lus). Résultat :
  * CHOIX : trustno1-coder/VIctory-NVIDIA (GPL-3.0, commit 033be27f du 2026-11-26 03:41). Spécialisé NVIDIA, combine avatarmanz + fxceless + ScarlettPort + d'autres. Mesuré : RTX 5070 12 Go / 9800X3D / 64 Go RAM, prologue 20–22 fps (Series S, demi-résolution, `scripts\play-nvidia.ps1`). A son xtask policy, pas de dossier diagnostics/. Limites : testé avec 64 Go de RAM ; ses patchs de résolution visent l'exécutable 1.0.2.1 du jeu ; Smooth Motion (génération d'images NVIDIA) gonfle la fluidité, pas les vrais fps.
  * Autres : avatarmanz/VIctory-7900XTX (1 556 étoiles, le plus actif, AMD, ~22 fps 7900 XTX, recommande 64 Go RAM) ; fxceless (= notre upstream, 7–12 fps) ; PastiiJay/klbf-lab (AMD 9070 XT, 14 fps moy.) ; TaynanGT/Vice-Tay (RTX 4090, 21 fps S, mémoire qui grimpe à 52 Go) ; edsuuu/VI-BR-PORT (4060 Ti 8 Go : ~12 fps Series S) ; Dolberthi/neptune… (vise 16 Go RAM / 8 Go VRAM mais aucun fps mesuré, à surveiller pour ses idées de réduction mémoire).
  * PLAN : Session 3 = branche `nvidia` partant de trustno1-coder 033be27f, audit, build, mesure chez moi. main reste intacte jusqu'à décision.

## Pour la prochaine conversation chef de projet
- Session 2 en cours (fusion upstream). Au « done » : vérifier que la fusion est sans conflit inattendu, que `cargo xtask policy` passe sur les 15 fichiers diagnostics/ d'upstream (journaux « scrubbed » : j'en ai lu un extrait, ils ne contiennent que des sorties de la couche + hashes, pas de données du jeu ; à re-vérifier), que les échecs fmt/clippy ne sont pas plus nombreux que ceux d'upstream, et comparer les fps 1re pièce cache chaud 253 vs 279.
- Ne JAMAIS mettre le chemin de ma copie du jeu dans un fichier du dépôt (il est dans victory.toml, ignoré par git).
