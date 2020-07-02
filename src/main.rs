use std::io;
//use std::any::type_name;

const L: usize = 6;
const C: usize = 7;
const INIT_CHAR: char = '0';

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    let mut grid = init_grille();
    // println!("{:?}", grid);
    // grid[0][6] = 'Y';
    //println!("{:?}", grid[3]);
    show_grille(&grid);
    let mut tour = 1;

    verification_puissance(&grid);

    loop {
        let mut piece: char;
        let mut puissance: &str;

        puissance = verification_puissance(&grid);

        match puissance {
            "Puissance_Y" => {println!("!!!! Joueur 1 GAGNE le match !!!!"); break;},
            "Puissance_U" => {println!("!!!! Joueur 2 GAGNE le match !!!!"); break;},
            _ => ()
        }

        if tour % 2 == 0 {
            println!("Joueur 2 !!");
            piece = 'U';
        } else {
            println!("Joueur 1 !!");
            piece = 'Y';
        }

        let (coup, action) = demande_coup();

        if action == 'q'.to_string() {
            break;
        }
        let position = calcul_position(&grid, &coup);
        println!("{}", position);
        grid[position as usize][coup as usize] = piece;

        show_grille(&grid);

        // verification_puissance(&grid);

        tour += 1;

    }


}

fn init_grille() -> Vec<Vec<char>> {
    //let mut grid: [[u8; 4]; 2] = [[0; 4]; 2];
    //let mut grid = [[0 as u8; C] ; L];
    let mut grid = vec![vec![INIT_CHAR; C]; L];

    grid
}

fn show_grille(grid: &[Vec<char>]) {
    for l in grid.iter() {
        println!("{:?}", l);
    }
    // for (i, row) in grid.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         println!("{}", col);
    //     }
    // }
}

fn demande_coup() -> (i8, String) {
    println!("Numero de action: ");
    println!("q pour quitter !");

    let mut action = String::new();
    io::stdin().read_line(&mut action)
        .ok()
        .expect("failed to read line");

    let coup: i8 = action.trim().parse()
                                .expect("whoops !");

    let test = action.trim().parse::<f64>();
    match test {
        Ok(ok) => action = "ACT_GAME".to_string(),
        Err(e) => action = "ACT_QUIT".to_string(),
    }


    // let action: u8 = action.trim().parse()
    //     .ok()
    //     .expect("Please type a number!");

    // print_type_of(&action);

    (coup, action)
}

fn calcul_position(grid: &[Vec<char>], coup: &i8) -> u8 {
    // for (c, col) in grid.iter().enumerate() {
    //     for (l, li) in col.iter().enumerate() {
    //         let mut piece = grid[c][l];
    //         println!("{:?}", piece);
    //     }
    //     println!("----------------------------");
    // }

    let mut position: u8 = 0;

    for p in 0..=5 {
        let piece = grid[p][*coup as usize];
        println!("la case contient: {}", piece);
        if piece != '0' {
            position = p as u8 - 1;
            println!("{}", position);
            break;
        } else {
            position = p as u8;
        }
    }
    position
}


fn verification_puissance(grid: &[Vec<char>]) -> &str {

    let mut buffer_verif: Vec<char> = Vec::new();
    let mut y_compteur = 0;
    let mut u_compteur = 0;

    // Verification des colonnes

    for c in 0..=6 {
        for l in (0..=5).rev() {
            // println!("{}{}", c, l);
            // println!("{}", grid[l][c]);


            // println!("{:?}", buffer_verif);
            // println!("{:?} {} {}", buff_i, grid[l][c], buffer_verif[buff_i-1]);

            if grid[l][c] == 'Y' {
                y_compteur+=1;
                buffer_verif.push(grid[l][c]);
                if y_compteur == 4 {println!("Puissance du Y"); return "Puissance_Y"; }
            } else {
                y_compteur = 0;
            }

            if grid[l][c] == 'U' {
                u_compteur+=1;
                buffer_verif.push(grid[l][c]);
                if u_compteur == 4 {println!("Puissance du U"); return "Puissance_U"; }
            } else {
                u_compteur = 0;
            }

            // if grid[l][c] != buffer_verif[buff_i-1] {

            // }
        }
        buffer_verif = vec![];
        y_compteur = 0;
        u_compteur = 0;
    }

    // Verification des lignes

    buffer_verif = vec![];

    for c in 0..=5 {
        for l in 0..6 {
            // println!("{}{}", c, l);
            // println!("{}", grid[l][c]);


            // println!("{:?}", buffer_verif);
            // println!("{:?}", buff_i);

            if grid[c][l] == 'Y' {
                y_compteur+=1;
                buffer_verif.push(grid[c][l]);
                if y_compteur == 4 {println!("Puissance du Y"); return "Puissance_Y"; }
            } else {
                y_compteur = 0;
            }

            if grid[c][l] == 'U' {
                u_compteur+=1;
                buffer_verif.push(grid[c][l]);
                if u_compteur == 4 {println!("Puissance du U"); return "Puissance_U"; }
            } else {
                u_compteur = 0;
            }

        }
        buffer_verif = vec![];
        y_compteur = 0;
        u_compteur = 0;
    }

    // ******* Verification des diagonales **************** //

    buffer_verif = vec![];

    let largeur = 7;
    let hauteur = 6;

    for l in 0..=largeur+hauteur-2 {
        for c in 0..=l {
            let i: i16 = l - c;
            if i < hauteur && c < largeur {
                // println!("i: {}", i);
                // println!("diag: {}", grid[i as usize][c as usize]);
                if grid[i as usize][c as usize] == 'Y' {
                    y_compteur+=1;
                    buffer_verif.push(grid[i as usize][c as usize]);
                    if y_compteur == 4 {println!("Puissance du Y"); return "Puissance_Y"; }
                } else {
                    y_compteur = 0;
                }
                if grid[i as usize][c as usize] == 'U' {
                    u_compteur+=1;
                    buffer_verif.push(grid[i as usize][c as usize]);
                    if u_compteur == 4 {println!("Puissance du U"); return "Puissance_U"; }
                } else {
                    u_compteur = 0;
                }
            }
        }
        buffer_verif = vec![];
        y_compteur = 0;
        u_compteur = 0;
    }

    let mut rev_grid = vec![];

    for l in 0..=5 {
        let mut rev_row = grid[l].clone();
        rev_row.reverse();
        // println!("{:?}", rev_row);
        rev_grid.push(rev_row);
    }

    // println!("{:?}", rev_grid);


    buffer_verif = vec![];

    for l in 0..=largeur+hauteur-2 {
        for c in 0..=l {
            let i: i16 = l - c;
            if i < hauteur && c < largeur {
                // println!("i: {}", i);
                // println!("diag: {}", rev_grid[i as usize][c as usize]);
                if rev_grid[i as usize][c as usize] == 'Y' {
                    y_compteur+=1;
                    buffer_verif.push(rev_grid[i as usize][c as usize]);
                    if y_compteur == 4 {println!("Puissance du Y"); return "Puissance_Y"; }
                } else {
                    y_compteur = 0;
                }
                if rev_grid[i as usize][c as usize] == 'U' {
                    u_compteur+=1;
                    buffer_verif.push(rev_grid[i as usize][c as usize]);
                    if u_compteur == 4 {println!("Puissance du U"); return "Puissance_U"; }
                } else {
                    u_compteur = 0;
                }
            }
        }
        buffer_verif = vec![];
        y_compteur = 0;
        u_compteur = 0;
    }


    "Aucun Gagnant"
}
