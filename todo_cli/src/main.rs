use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!();
        println!("=============================");
        println!("        To-do CLI Rust       ");
        println!("=============================");
        println!("1) Ajouter une tâche");
        println!("2) Afficher les tâches");
        println!("3) Compléter la dernière tâche");
        println!("4) Quitter");
        print!("Ton choix : ");
        // On flush pour que le texte s'affiche avant la lecture
        io::stdout().flush().expect("Erreur de flush");

        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let choix = choix.trim();

        if choix == "1" {
            ajouter_tache_interactive(&mut todos);
        } else if choix == "2" {
            afficher_taches(&todos);
        } else if choix == "3" {
            completer_derniere_tache(&mut todos);
        } else if choix == "4" {
            println!("Au revoir !");
            break;
        } else {
            println!("Choix invalide, réessaie.");
        }
    }
}

fn ajouter_tache_interactive(liste: &mut Vec<String>) {
    println!();
    println!("--- Ajouter une nouvelle tâche ---");
    print!("Description de la tâche : ");
    io::stdout().flush().expect("Erreur de flush");

    let mut saisie = String::new();
    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur de lecture");

    let saisie = saisie.trim();

    if saisie.is_empty() {
        println!("La tâche est vide, rien n'a été ajouté.");
    } else {
        // On crée un String à partir de la saisie &str
        let tache = String::from(saisie);
        ajouter_tache(liste, tache);
        println!("Tâche ajoutée !");
    }
}

fn ajouter_tache(liste: &mut Vec<String>, tache: String) {
    liste.push(tache);
}

fn afficher_taches(liste: &Vec<String>) {
    println!();
    println!("--- Mes tâches ---");

    if liste.is_empty() {
        println!("(aucune tâche pour l'instant)");
        return;
    }

    let mut index: usize = 1;
    for t in liste {
        println!("{index}. {t}");
        index += 1;
    }
}

fn completer_derniere_tache(liste: &mut Vec<String>) {
    let tache = liste.pop();

    if let Some(t) = tache {
        println!("Tâche complétée : {t}");
    } else {
        println!("Aucune tâche à compléter.");
    }
}