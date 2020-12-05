use rand::Rng;
use colored::Colorize;


fn main() {
    let person = vec!["Player1", "Player2"];


    let white_items = vec!["Armor Piercing Rounds", "Backup Magazine", "Fresh Meat",
                           "Rusted Key", "Topaz Broach", "Sticky Bomb", "Focus Crystal",
                           "Repulsion Armor Plate", "Energy Drink", "Bundle of Fireworks",
                           "Stun Grenade", "Gasoline", "Medkit", "Personal Shield Generator",
                           "Cautious Slug", "Warbanner", "Tri-Tip Dagger", "Crowbar",
                           "Bustling Fungus", "Pauls Goat Hoof", "Lens-Makers Glasses",
                           "Monster Tooth", "Tougher Times", "Soldiers Syringe"];

    let green_items = vec!["AtG Missile", "Bandolier", "Berzerkers Pauldron",
                           "Chronobauble", "Death Mark", "Fuel Cell", "Ghor's Tome",
                           "Harvester's Scythe", "Hopoo Feather", "Infusion", "Kjaro's Band",
                           "Leeching Seed", "Lepton Daisy", "Old Guillotine", "Old War Stealth kit",
                           "Predatory Instincts", "Razorwire", "Red Whip", "Rose Buckler",
                           "Runald's Band", "Squid Polyp", "Ukulele", "War Horn", "Wax Quail",
                           "Will-o'-the-wisp"];

    let red_items = vec!["57 Leaf Clover", "Aegis", "Alien Head", "Brainstalks",
                         "Brilliant Behemoth", "Cerimonial Dagger", "Defensive Microbots",
                         "Dios Best Friend", "Frost Relic", "H3AD-5T v2", "Happiest Mask",
                         "Hardlight Afterburner", "Intrestellar Desk Plant", "N'kuhana's Opinion",
                         "Rejuvination Rack", "Resonacne Disc", "Sentient Meat Hook",
                         "Shattering Justice", "Soulbound Catalyst", "Unstable Tesla Coil",
                         "Wake of Vultures"];

    let gold_items = vec!["Artifact Key", "Genesis Loop", "Halcyon Seed",
                          "Irradiant Pearl", "Little Disciple", "Mired Urn", "Molten Perforator",
                          "Pearl", "Queen's Gland", "Shatterspleen", "Titanic Knurl"];

    let white_item_number = white_items.len();
    let green_item_number = green_items.len();
    let red_item_number = red_items.len();
    let gold_item_number = gold_items.len();
    let person_number = person.len();

    let mut rng = rand::thread_rng();

    for x in 0..person_number {
        let random_white = rng.gen_range(0, white_item_number);
        let random_green = rng.gen_range(0, green_item_number);
        let random_red = rng.gen_range(0, red_item_number);
        let random_gold = rng.gen_range(0, gold_item_number);

        println!("\n{}'s items:", person[x]);
        println!("White: {}", white_items[random_white].white());
        println!("Green: {}", green_items[random_green].green());
        println!("Red: {}", red_items[random_red].red());
        println!("Gold: {}", gold_items[random_gold].yellow());
    }
}

