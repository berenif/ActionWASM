# Roguelike ARPG Tuning Tables

## Combat Timing Tables

### Animation Frames (60 FPS)
| Action | Startup | Active | Recovery | Total | Cancelable |
|--------|---------|--------|----------|-------|------------|
| Light Attack | 8f | 4f | 6f | 18f (0.3s) | Frame 12+ |
| Heavy Attack | 20f | 8f | 20f | 48f (0.8s) | Frame 28+ |
| Dash | 3f | 12f | 9f | 24f (0.4s) | Never |
| Parry | 3f | 9f | 18f | 30f (0.5s) | Frame 20+ |
| Dodge Roll | 5f | 15f | 10f | 30f (0.5s) | Frame 25+ |
| Skill Cast | 10f | 5f | 15f | 30f (0.5s) | Frame 15+ |

### Enemy Attack Telegraphs
| Enemy Type | Telegraph | Active | Recovery | Total Time |
|------------|-----------|--------|----------|------------|
| Melee Common | 0.5s | 0.2s | 0.3s | 1.0s |
| Melee Elite | 0.8s | 0.3s | 0.4s | 1.5s |
| Ranged Common | 0.7s | 0.1s | 0.5s | 1.3s |
| Ranged Elite | 1.0s | 0.1s | 0.6s | 1.7s |
| AOE Attack | 1.5s | 0.5s | 0.5s | 2.5s |
| Boss Attack | 2.0s | 0.5s | 1.0s | 3.5s |

---

## Damage Scaling Tables

### Player Damage Output
| Room # | Base DMG | Expected DPS | TTK Common | TTK Elite | TTK Boss |
|--------|----------|--------------|------------|-----------|----------|
| 1-5 | 100 | 100/s | 2s | 10s | - |
| 6-10 | 150 | 200/s | 1s | 7s | - |
| 11-15 | 250 | 400/s | 0.5s | 5s | 180s |
| 16-20 | 400 | 800/s | 0.25s | 3s | 120s |
| 21+ | 600 | 1500/s | Instant | 2s | 90s |

### Enemy Health Pools
| Enemy Type | Room 1-5 | Room 6-10 | Room 11-15 | Room 16-20 |
|------------|----------|-----------|------------|------------|
| Common | 200 HP | 300 HP | 400 HP | 500 HP |
| Elite | 1000 HP | 2000 HP | 3500 HP | 5000 HP |
| Miniboss | 3000 HP | 6000 HP | 10000 HP | 15000 HP |
| Boss | - | 15000 HP | 25000 HP | 40000 HP |

### Enemy Damage Output
| Enemy Type | Room 1-5 | Room 6-10 | Room 11-15 | Room 16-20 |
|------------|----------|-----------|------------|------------|
| Common | 20 DMG | 30 DMG | 50 DMG | 80 DMG |
| Elite | 40 DMG | 60 DMG | 100 DMG | 150 DMG |
| Miniboss | 60 DMG | 100 DMG | 150 DMG | 250 DMG |
| Boss | - | 80 DMG | 120 DMG | 200 DMG |

---

## Economy Tables

### Room Rewards
| Room Type | Gold Min | Gold Max | Soul Chance | Item Rarity |
|-----------|----------|----------|-------------|-------------|
| Common | 20 | 40 | 5% | 80% Common, 20% Rare |
| Elite | 60 | 100 | 15% | 50% Common, 40% Rare, 10% Epic |
| Miniboss | 150 | 250 | 30% | 30% Rare, 50% Epic, 20% Legendary |
| Boss | 300 | 500 | 100% | 100% Epic + Legendary choice |
| Secret | 50 | 150 | 10% | 60% Rare, 40% Epic |
| Challenge | 100 | 200 | 20% | 40% Rare, 60% Epic |

### Shop Pricing
| Item Tier | Base Cost | Biome 1 | Biome 2 | Biome 3 | Biome 4 |
|-----------|-----------|---------|---------|---------|---------|
| Common | 50g | 50g | 75g | 100g | 150g |
| Rare | 150g | 150g | 225g | 300g | 450g |
| Epic | 400g | 400g | 600g | 800g | 1200g |
| Legendary | 1000g | - | 1000g | 1500g | 2000g |
| Heal (25%) | 40g | 40g | 60g | 80g | 120g |
| Heal (Full) | 200g | 200g | 300g | 400g | 600g |
| Remove Boon | 100g | 100g | 150g | 200g | 300g |

### Reroll Costs
| Reroll # | Cost | Cumulative |
|----------|------|------------|
| 1st | 25g | 25g |
| 2nd | 50g | 75g |
| 3rd | 75g | 150g |
| 4th | 100g | 250g |
| 5th+ | 150g | 400g+ |

---

## Power Scaling Tables

### Boon Rarity Multipliers
| Rarity | Power Multi | Proc Chance | Cooldown Reduction | Stack Limit |
|--------|-------------|-------------|-------------------|-------------|
| Common | 1.0x | 10% | 10% | 3 |
| Rare | 1.5x | 15% | 15% | 5 |
| Epic | 2.0x | 20% | 20% | 7 |
| Legendary | 3.0x | 30% | 30% | 10 |
| Mythic | 5.0x | 50% | 50% | No limit |

### Stat Scaling Breakpoints
| Stat | Soft Cap | Hard Cap | Diminishing Returns |
|------|----------|----------|-------------------|
| Crit Chance | 50% | 75% | -50% efficiency after soft cap |
| Crit Damage | 200% | 400% | -75% efficiency after soft cap |
| Attack Speed | 150% | 200% | -80% efficiency after soft cap |
| Movement Speed | 150% | 200% | -60% efficiency after soft cap |
| Cooldown Reduction | 40% | 60% | -90% efficiency after soft cap |
| Damage Reduction | 50% | 75% | -95% efficiency after soft cap |

---

## Risk/Reward Tables

### Elite Modifiers
| Modifier | HP Multi | DMG Multi | Reward Multi | Special Effect |
|----------|----------|-----------|-------------|----------------|
| Armored | 1.5x | 1.0x | 1.5x | 50% damage reduction until broken |
| Berserker | 1.0x | 2.0x | 1.5x | +50% attack speed |
| Regenerating | 2.0x | 1.0x | 2.0x | 5% HP/sec regen |
| Shielded | 1.0x | 1.0x | 1.5x | Immune until shield breaks |
| Vampiric | 1.2x | 1.2x | 1.8x | Heals 20% damage dealt |
| Explosive | 0.8x | 1.5x | 2.0x | Explodes on death for 200% HP as damage |

### Curse Effects
| Curse Type | Penalty | Bonus | Duration |
|------------|---------|-------|----------|
| Fragile | -30% Max HP | +40% Damage | Permanent |
| Slow | -25% Move Speed | +30% Attack Speed | 5 rooms |
| Blind | -50% Vision Range | +50% Crit Chance | 3 rooms |
| Cursed | -1 HP/sec | +100% Gold Find | Until healed |
| Marked | Enemies target you first | +25% All Stats | 10 rooms |
| Doomed | Die in one hit | +200% Damage | 1 room |

---

## Spawn Tables

### Room Density
| Room # | Min Enemies | Max Enemies | Elite Chance | Configuration |
|--------|-------------|-------------|--------------|---------------|
| 1-3 | 2 | 4 | 0% | Singles and pairs |
| 4-6 | 3 | 6 | 10% | Small groups |
| 7-10 | 4 | 8 | 20% | Mixed groups |
| 11-15 | 5 | 10 | 30% | Large groups + elite |
| 16-20 | 6 | 12 | 40% | Swarms + multiple elites |
| 21+ | 8 | 15 | 50% | Endless waves |

### Wave Spawning
| Wave # | Delay | Enemy Count | Composition |
|--------|-------|-------------|-------------|
| Initial | 0s | 40% of total | Basic enemies |
| Wave 2 | 5s | 30% of total | Mixed types |
| Wave 3 | 10s | 20% of total | Elite + support |
| Wave 4 | 15s | 10% of total | Reinforcements |

---

## Cooldown Tables

### Ability Cooldowns
| Ability Type | Base CD | Min CD | CD per Rank | Global CD |
|--------------|---------|--------|-------------|-----------|
| Mobility | 4s | 1.5s | -0.5s | 0.5s |
| Damage | 6s | 2s | -0.8s | 1s |
| Defensive | 8s | 3s | -1s | 1s |
| Ultimate | 30s | 10s | -4s | 2s |
| Consumable | 10s | 5s | -1s | 1s |

### Status Effect Durations
| Effect | Base Duration | Max Stacks | Stack Duration | Refresh Rule |
|--------|---------------|------------|----------------|--------------|
| Burn | 3s | 5 | +1s per stack | Refresh to max |
| Poison | 5s | 10 | +0.5s per stack | Add duration |
| Freeze | 1.5s | 1 | - | Replace |
| Stun | 1s | 1 | - | Immunity 2s |
| Slow | 2s | 3 | Same duration | Refresh |
| Bleed | 4s | 10 | Same duration | Refresh |

---

## Meta Progression Tables

### Experience Requirements
| Level | XP Required | Total XP | Reward Type |
|-------|-------------|----------|-------------|
| 1 | 100 | 100 | Starting weapon |
| 2 | 200 | 300 | Passive slot |
| 3 | 400 | 700 | New character |
| 4 | 700 | 1400 | Starting boon |
| 5 | 1000 | 2400 | New weapon |
| 10 | 2500 | 15000 | Legendary unlock |
| 20 | 5000 | 65000 | Ascension mode |
| 50 | 10000 | 350000 | True ending |

### Ascension Difficulty
| Ascension | Enemy HP | Enemy DMG | Player HP | Special Rules |
|-----------|----------|-----------|-----------|---------------|
| 0 (Base) | 100% | 100% | 100% | None |
| 1 | 110% | 110% | 100% | Elites spawn earlier |
| 2 | 120% | 120% | 90% | Fewer heals |
| 3 | 130% | 130% | 90% | Shop prices +25% |
| 4 | 140% | 140% | 80% | Boss has 4 phases |
| 5 | 150% | 150% | 80% | No revives |
| 10 | 200% | 200% | 60% | All enemies are elite |
| 15 | 300% | 300% | 50% | Time limit per room |
| 20 | 500% | 500% | 40% | One hit mode |

---

## Performance Targets

### Frame Budget (16.67ms @ 60fps)
| System | Budget | Priority |
|--------|--------|----------|
| Input | 1ms | Critical |
| Physics | 3ms | High |
| AI | 2ms | Medium |
| Rendering | 8ms | High |
| UI | 1ms | Medium |
| Audio | 0.5ms | Low |
| Networking | 1ms | Variable |
| Buffer | 0.17ms | - |

### Memory Limits
| Platform | RAM Target | VRAM Target | Storage |
|----------|------------|-------------|---------|
| Mobile | 2GB | 1GB | 500MB |
| Console | 4GB | 4GB | 2GB |
| PC Low | 4GB | 2GB | 2GB |
| PC High | 8GB | 6GB | 4GB |

---

## Loot Tables

### Drop Rates by Source
| Source | Common | Rare | Epic | Legendary | Mythic |
|--------|--------|------|------|-----------|--------|
| Common Enemy | 70% | 25% | 5% | 0% | 0% |
| Elite Enemy | 40% | 40% | 18% | 2% | 0% |
| Miniboss | 20% | 40% | 35% | 5% | 0% |
| Boss | 0% | 30% | 50% | 19% | 1% |
| Chest | 50% | 35% | 13% | 2% | 0% |
| Secret | 30% | 40% | 25% | 5% | 0% |
| Shop | 40% | 35% | 20% | 5% | 0% |

### Pity System
| Rarity | Guaranteed After | Reset On Drop |
|--------|-----------------|---------------|
| Rare | 5 commons | Yes |
| Epic | 10 rares | Yes |
| Legendary | 20 epics | Yes |
| Mythic | 50 legendaries | No |

---

## Build Synergy Multipliers

### Damage Type Combinations
| Primary | Secondary | Multiplier | Special Effect |
|---------|-----------|------------|----------------|
| Fire | Oil | 1.5x | Spreads to nearby enemies |
| Ice | Lightning | 1.4x | Chain freezes |
| Poison | Bleed | 1.6x | DoT stacks faster |
| Physical | Armor Break | 2.0x | Ignores defense |
| Magic | Curse | 1.8x | Reduces resistances |

### Archetype Scaling
| Archetype | Early (1-5) | Mid (6-15) | Late (16-20) | Scaling Type |
|-----------|-------------|------------|--------------|--------------|
| Glass Cannon | 1.0x | 2.0x | 4.0x | Exponential |
| Tank | 1.2x | 1.5x | 2.0x | Linear |
| Summoner | 0.8x | 1.5x | 3.0x | Late bloom |
| Controller | 1.0x | 1.8x | 2.5x | Steady |
| Speed | 1.1x | 1.6x | 3.5x | Momentum-based |