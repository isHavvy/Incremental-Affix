use item::SuffixOrPrefix;

mod item;

fn main() {
    let implicits = item::initialize_implicits();
    let bases = item::initialize_bases();

    let prefixes = item::initialize_prefixes();
    let suffixes = item::initialize_suffixes();
    let drop_tables = item::initialize_drop_tables(&bases, prefixes, suffixes);

    let bases_table = drop_tables.get::<item::StorageBasesDropTable>().unwrap();
    let base_ix = bases_table.random();

    let mut storage = item::AffixiveItem::new(&bases, &implicits, base_ix, item::Quality::Quality(1));
    
    let suffix_or_prefix_table = drop_tables.get::<item::SuffixOrPrefixDropTable>().unwrap();
    match suffix_or_prefix_table.random() {
        SuffixOrPrefix::Prefix => {
            let prefix_table = drop_tables.get::<item::InventoryModifierPrefixes>().unwrap();
            let mut prefix = prefix_table.random();
            prefix.randomize_actual();

            let _ = storage.try_push_prefix(prefix);
        },

        SuffixOrPrefix::Suffix => {
            let suffix_table = drop_tables.get::<item::InventoryModifierSuffixes>().unwrap();
            let mut suffix = suffix_table.random();
            suffix.randomize_actual();

            let _ = storage.try_push_suffix(suffix);
        }
    }

    println!("{}", storage.display(&bases[..]));
}