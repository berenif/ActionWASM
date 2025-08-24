# Roguelike ARPG Design Bible

## The Core Loop

**Explore → Fight → Choose → Power-up → Push your luck → Escalate → Cash-out → Reset**

---

## 1. EXPLORE

### Goal
Read the space fast, spot threats, sniff out rewards.

### Fun Comes From
- Clear landmarks that orient the player
- Light fog of war that teases without frustrating
- Tiny secrets that pay off often (10-20% of rooms)

### Design Knobs
- **Room shapes**: Arena (circular combat), Gauntlet (linear challenge), Puzzle (environmental)
- **Hazards**: Environmental dangers that affect both player and enemies
- **Discovery events**: Breakable walls, hidden switches, glowing cracks
- **Safe peeks**: Doors show enemy type icons, corners allow tactical assessment

### Implementation Checklist
- [ ] Minimap shows cleared rooms and hints at unexplored
- [ ] Audio cues for nearby secrets (subtle hum, chains rattling)
- [ ] Visual language for interactables (gold shimmer = loot, purple glow = curse)
- [ ] 3-second room preview on entry before enemies activate

---

## 2. FIGHT

### Goal
Express skill under pressure.

### Fun Comes From
- Snappy movement with zero input lag
- Readable telegraphs with consistent timing
- Reliable i-frames or guards that reward mastery

### Design Knobs
- **Enemy mix**: Ranged forces movement, melee creates space pressure, support buffs threats
- **Crowd control windows**: 0.5s stun on parry, 1s slow on frost proc
- **Breakable systems**: Armor bars, posture meters, stagger thresholds

### Implementation Checklist
- [ ] Input buffer: 100ms window for chained actions
- [ ] Cancel windows: Can dash out of 80% of animations
- [ ] Telegraph colors: Red = damage, Yellow = stun, Purple = debuff
- [ ] Hitbox clarity: Show exact frames and zones in practice mode

### Combat Feel Standards
```
Light Attack: 0.3s animation, 0.1s recovery
Heavy Attack: 0.8s animation, 0.3s recovery  
Dash: 0.2s duration, 0.4s cooldown
Parry Window: 0.15s active frames
```

---

## 3. CHOOSE

### Goal
Draft the run's identity in small bites.

### Fun Comes From
- Three solid options, never a non-choice
- Each option shows clear tradeoffs
- Hints of future combos in the text

### Design Knobs
- **Limited slots**: 3 active abilities, 6 passive boons, 1 curse
- **Exclusions**: Fire locks out ice, speed locks out armor
- **UI clarity**: Side-by-side comparison, DPS calculator, synergy icons

### Choice Architecture
```
ALWAYS OFFER:
- One option that enhances current build
- One option that pivots to new strategy  
- One wildcard with unique mechanic

NEVER OFFER:
- Three of the same damage type
- All defensive or all offensive
- Choices with hidden downsides
```

---

## 4. POWER-UP

### Goal
Feel stronger NOW, not later.

### Fun Comes From
- Immediate breakpoints crossed
- Visible stat jumps on UI
- New verb unlocked instantly

### Design Knobs
- **Boons**: Direct combat modifiers (25% crit, chain lightning)
- **Relics**: Passive rule changes (double jump, magnetize gold)
- **Affixes**: Weapon properties (+2 projectiles, lifesteal)
- **Pets**: Autonomous helpers with clear AI
- **Cooldown tweaks**: -20% dash CD crosses key threshold

### Power Curve Targets
```
Room 1-5:   Base power
Room 6-10:  150% power (one-shot commons)
Room 11-15: 250% power (two-shot elites)
Room 16-20: 400% power (combo chains clear screens)
Boss:       Test of build synergy, not raw stats
```

---

## 5. PUSH YOUR LUCK

### Goal
Opt into risk for better rewards.

### Fun Comes From
- Predictable risk with clear communication
- Juicy upside that tempts greed
- Escape hatches for when things go wrong

### Design Knobs
- **Elite rooms**: 2x rewards, 3x enemy HP, guaranteed rare drop
- **Curse shrines**: -20% max HP for +40% damage
- **Timed chests**: 30 seconds for triple loot
- **Blood doors**: Spend 25% current HP to enter

### Risk/Reward Table
| Risk Level | Cost | Reward | Escape Option |
|------------|------|--------|---------------|
| Low | 10% HP | Common boon | Always available |
| Medium | 25% HP or 1 slot | Rare boon | Costs gold |
| High | 50% HP or curse | Epic boon + relic | One-time use item |
| Extreme | 75% HP + curse | Legendary | None |

---

## 6. ESCALATE

### Goal
New problems, not just bigger numbers.

### Fun Comes From
- Biomes that change core rules
- Enemy variants with new patterns
- Fresh hazards that test builds differently

### Design Knobs
- **Density pacing**: 3 → 5 → 8 → 12 enemies per room
- **Elite modifiers**: Shielded, Regenerating, Explosive, Vampiric
- **Miniboss interrupts**: Every 5 rooms, tests specific build aspect

### Biome Progression
```
Biome 1 (Tutorial): Basic enemies, no hazards
Biome 2 (Skill check): Projectiles, poison pools  
Biome 3 (Build test): Shields, teleporters
Biome 4 (Mastery): All mechanics, time pressure
Biome 5 (Victory lap): Power fantasy if built right
```

---

## 7. CASH-OUT

### Goal
Spend smart, heal, or double-down.

### Fun Comes From
- Tense shop decisions
- Meaningful removal options
- Reforge gambles

### Design Knobs
- **Two currencies**: Gold (common) and Souls (rare)
- **Sinks**: Remove bad boons, reroll shop, heal to full
- **Safety valves**: Guaranteed small heal, insurance items

### Shop Economy
```
COSTS (in gold):
- Common boon: 50
- Rare boon: 150
- Epic boon: 400
- Remove boon: 100
- Reroll shop: 25 (increases by 25 each use)
- Small heal (25%): 40
- Full heal: 200
- Mystery box: 75

SOUL COSTS:
- Legendary boon: 3 souls
- Weapon unlock: 5 souls
- Permanent upgrade: 10 souls
```

---

## 8. RESET

### Goal
Fail fast, try again with different angle.

### Fun Comes From
- Instant restart (< 3 seconds)
- Seed variety ensures fresh runs
- Short early rooms to regain flow

### Design Knobs
- **Post-run recap**: Damage dealt/taken, build summary, unlock progress
- **Meta drip**: 5-10% permanent progress per run
- **Unlock tease**: "You were 2 kills from unlocking X"

### Fail States That Feel Fair
- Clear what killed you (damage recap)
- Could see it coming (HP bars, warnings)
- Know what to do differently
- Excited to try new approach

---

## Systems That Make It Sing

### Randomness That Respects Intent

**Curated RNG**
- Small pools per biome (20-30 items max)
- Pity timers (guarantee rare every 5 choices)
- Smart rerolls (won't offer same twice)

**Choice Shape**
- 75% weighted toward your build
- 25% wildcard for variety
- Never completely random

**Guarantees**
- First room always has movement upgrade
- Shop before each boss
- At least one healing opportunity per biome

### Buildcraft and Synergies

**Core Archetypes**

| Archetype | Strengths | Weaknesses | Key Synergies |
|-----------|-----------|------------|---------------|
| Glass Cannon | Extreme damage | No defense | Lifesteal, one-shot protection |
| Tank | High survivability | Low damage | Thorns, revenge damage |
| Summoner | Hands-off combat | Vulnerable alone | Pet buffs, minion chains |
| Controller | Crowd management | Single target weak | Combo extenders, executions |
| Speed Demon | Extreme mobility | Low HP | Hit-and-run bonuses, momentum |

**Breakpoints to Design Around**
```
Commons: Die in 2 hits (feels smooth)
Elites: Die in 5-7 hits (tactical)
Miniboss: 15-20 hits (endurance test)
Boss: 40-60 hits (3-5 minute fight)

Player dodge: 2 uses before cooldown
Player HP: Survive 5 common hits
Player DPS: Clear room in 30-45 seconds
```

### Combat Feel Requirements

**Input Trust**
- Zero input lag (< 16ms)
- Buffer system for combos
- Cancel windows clearly defined
- Dash/dodge always responsive

**Readability**
- Damage numbers: White (normal), Yellow (crit), Red (player hurt)
- Status effects: Icon + timer + stack count
- Telegraph timing: 0.5s minimum, 2s maximum
- Screen shake: Light for hits, medium for crits, heavy for kills

**Pacing**
```
Combat burst: 10-15 seconds
Breathing room: 3-5 seconds  
Choice moment: 10-20 seconds
Travel time: 5-10 seconds
Full room: 30-45 seconds
```

---

## Level and Encounter Design

### Room Archetypes

**Arena (40% of rooms)**
- Circular or square space
- Enemies spawn in waves
- Environmental hazards on edges

**Gauntlet (40% of rooms)**
- Linear progression
- Timed challenges
- Platform elements

**Puzzle (20% of rooms)**
- Environmental solution
- Optional combat
- Hidden rewards

### Secrets Distribution
- 10% rooms: Obvious secret (cracked wall)
- 5% rooms: Hidden secret (invisible platform)
- 5% rooms: Puzzle secret (switch sequence)

---

## Boss Design Philosophy

### Purpose
Validate the build - test what player has assembled

### Fairness Rules
- Phase 1: Teaching phase (50% HP) - shows all attacks
- Phase 2: Combination phase (30% HP) - mixes patterns
- Phase 3: Desperation phase (20% HP) - new twists

### Rewards
- Guaranteed epic boon
- Choose from 3 legendary options
- Full heal
- Meta currency bonus

---

## Meta Progression

### Unlock Priority
1. **Horizontal unlocks first**: New weapons, starting boons
2. **Quality of life second**: Start with dash, inventory space
3. **Power last**: +5% damage, +10 starting HP

### Daily Hooks
- Seeded runs with fixed RNG
- Mutators (no dash, all elite, double speed)
- Leaderboards with replay viewing
- 15-20 minute average completion

---

## UX Polish Checklist

### Information Hierarchy
1. **Always visible**: HP, ability cooldowns, combo counter
2. **Combat only**: Damage numbers, buff timers, threat indicators
3. **On demand**: Full stats, build summary, encyclopedia

### Quality of Life
- [ ] One button restart from death
- [ ] Skip all cutscenes/dialogue option
- [ ] Colorblind modes for all telegraphs
- [ ] Damage log for last death
- [ ] Build save codes for sharing
- [ ] Practice room always accessible

---

## Anti-Fun Traps to Avoid

### Never Do This
- ❌ Off-screen damage with no warning
- ❌ Stun-locks longer than 1 second
- ❌ RNG that can completely break a run
- ❌ Mandatory grind for basic features
- ❌ Unclear scaling or hidden mechanics
- ❌ Unskippable tutorials after first time
- ❌ Builds that can't complete content
- ❌ Meta progression that trivializes skill

### Always Do This
- ✅ Every death teaches something
- ✅ Every choice matters
- ✅ Every build can win with skill
- ✅ Every run feels different
- ✅ Progress visible at all times
- ✅ Restart takes < 3 seconds
- ✅ Clear visual/audio feedback
- ✅ Respect player's time

---

## Quick Start Blueprint

### MVP Features (Week 1)
1. Movement and basic combat
2. 3 enemy types
3. 10 rooms
4. 5 boons
5. 1 boss

### Vertical Slice (Month 1)
1. Complete first biome
2. 3 weapon types
3. 20 boons with synergies
4. Shop system
5. Basic meta progression

### Full Loop (Month 3)
1. 3 biomes
2. 50+ boons and items
3. 5+ bosses
4. Daily challenges
5. Polish and balancing

---

## Success Metrics

### Engagement
- Average run time: 25-35 minutes
- Runs per session: 2-3
- Win rate: 5% (new players) → 30% (experienced)

### Progression Feel
- First win: 10-20 runs
- All content seen: 50-100 runs  
- Mastery achieved: 200+ runs

### Player Sentiment
- "Just one more run"
- "I almost had it"
- "I can't wait to try X build"
- "That death was my fault"