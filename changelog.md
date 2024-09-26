# [0.2.3] 2024-09-26

## added

- added readme

- added changelog

- added GetRandomItem trait

- added impl of GetRandomItem for Vec

- added impl of GetRandomItem for HashMap

- added RemoveRandomItem trait

- added impl of RemoveRandomItem for Vec

- added impl of RemoveRandomItem for HashMap

## Changed

- Pcg32::get_random_item() to use GetRandomItem

- Pcg32::remove_random_item() to use RemoveRandomItem

