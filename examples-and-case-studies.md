# Roguelike ARPG Examples & Case Studies

## Successful Implementation Examples

### Hades (Supergiant Games)
**What They Nailed:**
- **Instant Restart**: Death → Dialogue → Back in action (< 10 seconds)
- **Clear Telegraphs**: Red circles for AOE, exclamation marks for attacks
- **Choice Presentation**: Three boons, clear god themes, synergy hints
- **Meta Progression**: Story progression makes death meaningful

**Key Takeaway**: Every death advances story, making failure feel like progress

**Specific Mechanics:**
```
Room Clear → Choose Door (shows reward type) → 
Get Boon (3 choices) → Immediate power spike → 
Next room harder but you're stronger
```

---

### Risk of Rain 2 (Hopoo Games)
**What They Nailed:**
- **Time Pressure**: Difficulty scales with time, not just progression
- **Item Stacking**: Simple multipliers create exponential power
- **Visual Feedback**: Items physically appear on character
- **Co-op Integration**: Shared progression, individual choices

**Key Takeaway**: Time as currency creates constant tension

**Difficulty Scaling:**
```
Every 5 minutes:
- Enemy HP: +40%
- Enemy Damage: +20%
- Enemy Speed: +10%
- Spawn Rate: +30%
- Elite Chance: +5%
```

---

### Dead Cells (Motion Twin)
**What They Nailed:**
- **Movement Flow**: Chain rolls, climbs, and ground pounds
- **Weapon Variety**: 100+ weapons, each changes playstyle
- **Biome Paths**: Multiple routes with different rewards
- **Speed Incentive**: Timed doors with exclusive rewards

**Key Takeaway**: Movement IS combat - they're inseparable

**Speed Reward System:**
```
< 2 min to first boss: 20 cells + legendary weapon
< 8 min to second area: 40 cells + epic scroll
< 15 min to third area: 60 cells + stat boost
```

---

### Vampire Survivors (poncle)
**What They Nailed:**
- **Auto-Combat**: Remove execution, focus on positioning
- **Visual Chaos**: Screen-filling effects = power fantasy
- **Evolution System**: Combine items for super weapons
- **Session Length**: Exactly 30 minutes, perfect for "one more run"

**Key Takeaway**: Simplicity can create depth

**Evolution Example:**
```
Knife (base) + Bracer (passive) = Thousand Edge
- Knife: 1 projectile, 10 damage
- Thousand Edge: Infinite projectiles, 25 damage
- Visual: Screen full of knives
```

---

## Practical Implementation Examples

### Example 1: Telegraph System
```python
class TelegraphSystem:
    def create_telegraph(self, attack_type, position, duration):
        telegraph = {
            'melee': {
                'color': RED,
                'shape': 'arc',
                'duration': 0.5,
                'fade_in': 0.1
            },
            'ranged': {
                'color': YELLOW,
                'shape': 'line',
                'duration': 0.7,
                'fade_in': 0.2
            },
            'aoe': {
                'color': ORANGE,
                'shape': 'circle',
                'duration': 1.5,
                'fade_in': 0.3
            }
        }
        
        # Visual stages
        # 0-20%: Fade in
        # 20-80%: Full visibility
        # 80-90%: Flash warning
        # 90-100%: Execute attack
```

### Example 2: Boon Selection UI
```python
class BoonChoice:
    def present_choices(self, player_build):
        choices = []
        
        # 60% chance: Synergistic with current build
        if random() < 0.6:
            choices.append(get_synergistic_boon(player_build))
        
        # 30% chance: New build path
        if random() < 0.3:
            choices.append(get_pivot_boon(player_build))
        
        # 10% chance: Wildcard
        if random() < 0.1:
            choices.append(get_random_legendary())
        
        # Always show exactly 3
        while len(choices) < 3:
            choices.append(get_random_boon())
        
        return choices[:3]
```

### Example 3: Room Generation
```python
class RoomGenerator:
    def generate_room(self, room_number, biome):
        room = Room()
        
        # Determine room type
        if room_number % 5 == 0:
            room.type = 'shop'
        elif room_number % 10 == 0:
            room.type = 'boss'
        elif random() < 0.2:
            room.type = 'elite'
        elif random() < 0.1:
            room.type = 'treasure'
        else:
            room.type = 'combat'
        
        # Scale difficulty
        room.enemy_count = min(3 + room_number // 2, 12)
        room.enemy_level = 1 + room_number // 5
        
        # Add variety
        if random() < 0.1:
            room.add_hazard()
        if random() < 0.15:
            room.add_secret()
        
        return room
```

### Example 4: Damage Calculation
```python
def calculate_damage(base_damage, attacker_stats, defender_stats):
    # Base calculation
    damage = base_damage * attacker_stats.damage_multiplier
    
    # Critical hits
    if random() < attacker_stats.crit_chance:
        damage *= attacker_stats.crit_damage
        show_crit_effect()
    
    # Elemental bonuses
    if attacker_stats.element == weakness_of(defender_stats.element):
        damage *= 1.5
    
    # Defense reduction
    damage -= defender_stats.armor
    damage *= (1 - defender_stats.damage_reduction)
    
    # Minimum damage (always do something)
    damage = max(damage, 1)
    
    return round(damage)
```

---

## Common Pitfalls & Solutions

### Pitfall 1: Choice Paralysis
**Problem**: Too many boons to choose from
**Solution**: Always show exactly 3 options

**Bad Example:**
```
Choose 1 from 10 boons (overwhelming)
```

**Good Example:**
```
Choose 1 from 3 boons (clear decision)
Reroll available for cost (player agency)
```

### Pitfall 2: Unclear Synergies
**Problem**: Players don't know what works together
**Solution**: Visual tags and explicit text

**Bad Example:**
```
"Burning Blood: Increases damage"
(How much? When? With what?)
```

**Good Example:**
```
"Burning Blood: +40% damage to burning enemies
Tags: [Fire] [DoT]
Synergizes with: Flame Dash, Ignite"
```

### Pitfall 3: Death Feels Cheap
**Problem**: Player dies to off-screen projectile
**Solution**: Always telegraph from visible range

**Implementation:**
```python
def spawn_projectile(origin, target):
    if not is_on_screen(origin):
        # Create warning indicator at screen edge
        show_offscreen_indicator(origin.direction)
        delay(0.5)  # Give player time to react
    
    # Now spawn the actual projectile
    create_projectile(origin, target)
```

### Pitfall 4: RNG Ruins Runs
**Problem**: Never finding the item you need
**Solution**: Pity timers and guaranteed drops

**Implementation:**
```python
class LootSystem:
    def __init__(self):
        self.rolls_since_rare = 0
        self.rolls_since_epic = 0
    
    def get_loot(self):
        # Pity system
        if self.rolls_since_epic > 20:
            return get_epic_item()
        elif self.rolls_since_rare > 5:
            return get_rare_item()
        
        # Normal RNG with bad luck protection
        roll = random()
        if roll < 0.05:  # 5% epic
            self.rolls_since_epic = 0
            return get_epic_item()
        elif roll < 0.25:  # 20% rare
            self.rolls_since_rare = 0
            return get_rare_item()
        else:
            self.rolls_since_rare += 1
            self.rolls_since_epic += 1
            return get_common_item()
```

---

## Build Examples That Work

### The Glass Cannon Build
**Core Items:**
- Berserker's Rage: +100% damage, -50% max HP
- Glass Sword: Crits deal 5x damage but you die in one hit
- Momentum: +10% damage per room without taking damage

**Why It Works:**
- Clear risk/reward
- Skill expression through dodging
- Huge payoff for perfect play

**Counter-play:**
- Requires perfect positioning
- Weak to chip damage
- Needs escape tools

### The Proc Chain Build
**Core Items:**
- Chain Lightning: Attacks chain to 3 enemies
- Elemental Overload: 25% chance to trigger all elements
- Cascade: On-kill effects trigger twice

**Why It Works:**
- Visual spectacle (screen full of effects)
- Scales with enemy density
- Satisfying chain reactions

**Example Chain:**
```
Hit enemy → 
Lightning chains (3 targets) → 
Each proc fire (25% chance) → 
Fire spreads (area damage) → 
Kills trigger cascade → 
More lightning chains → 
Screen cleared
```

### The Tank Build
**Core Items:**
- Thorns: Return 50% damage taken
- Regeneration: Heal 2% max HP/second
- Fortress: Gain armor equal to missing HP%

**Why It Works:**
- Different playstyle (stand and fight)
- Consistent, not flashy
- Good for learning boss patterns

**Scaling:**
```
Early: Survive 5 hits → 7 hits → 10 hits
Mid: Heal between fights → Heal during fights
Late: Enemies kill themselves on your thorns
```

---

## Room Design Examples

### Arena Room (Combat Focus)
```
Layout: Circular arena, pillars for cover
Enemies: 3 melee, 2 ranged
Hazard: Rotating fire beams
Secret: Breakable wall behind pillar
Duration: 30-45 seconds
```

### Gauntlet Room (Movement Focus)
```
Layout: Long corridor with platforms
Enemies: Continuous spawns from behind
Hazard: Rising lava (forces forward movement)
Secret: Hidden platform up high
Duration: 60 seconds exactly
```

### Puzzle Room (Brain Focus)
```
Layout: Grid of pressure plates
Enemies: None initially, swarm if failed
Mechanic: Step on plates in sequence
Reward: Guaranteed rare item
Duration: No time limit
```

---

## Economy Balance Examples

### Shop Pricing That Works
```
Early Game Shop (Rooms 1-5):
- Common Boon: 50g (2 room clears)
- Heal (25%): 40g (2 room clears)
- Reroll: 25g (1 room clear)
Player typically has: 100-150g
Decision: Heal or power?

Mid Game Shop (Rooms 10-15):
- Rare Boon: 200g (3 room clears)
- Epic Boon: 500g (6 room clears)
- Remove Boon: 150g (2 room clears)
Player typically has: 400-600g
Decision: Quality or quantity?

Late Game Shop (Rooms 20+):
- Legendary Boon: 1000g (all current gold)
- Full Heal: 400g
- Reforge Item: 300g
Player typically has: 800-1200g
Decision: All-in or play safe?
```

---

## Boss Pattern Examples

### Phase-Based Boss Design
**Phase 1 (100-60% HP): Teaching**
```
- Attack 1: Single projectile (1s telegraph)
- Attack 2: Melee swipe (0.8s telegraph)
- Pattern: 1-1-2, repeat
- Player learns: Timing and tells
```

**Phase 2 (60-30% HP): Combination**
```
- Attack 1+2: Projectile during swipe
- Attack 3: AOE slam (1.5s telegraph)
- Pattern: 1+2-3-1-2-3, repeat
- Player learns: Multi-tasking
```

**Phase 3 (30-0% HP): Desperation**
```
- All previous attacks faster (-25% telegraph)
- Attack 4: Screen-wide wave (2s telegraph)
- Pattern: Random with tells
- Player learns: Mastery required
```

---

## UI/UX Examples

### Damage Number Display
```
Normal Hit: White, small font, floats up
Critical Hit: Yellow, large font, screen shake
Overkill: Red, huge font, explosion effect
Healing: Green, medium font, gentle float
DoT: Purple, tiny font, rapid ticks
```

### Health Bar Information Density
```
Player HP Bar Shows:
- Current/Max HP numbers
- Armor as grey overlay
- Recent damage as red flash
- Heal preview on pickup hover
- Status effects as icons below

Enemy HP Bar Shows:
- Health segments (for chunking)
- Armor/Shield as second bar
- Debuffs as small icons
- Stagger meter if applicable
- Level/Elite status
```

---

## Performance Optimization Examples

### Particle Pooling
```python
class ParticlePool:
    def __init__(self, size=1000):
        self.particles = [Particle() for _ in range(size)]
        self.active = []
        self.inactive = self.particles.copy()
    
    def spawn(self, position, type):
        if self.inactive:
            particle = self.inactive.pop()
            particle.reset(position, type)
            self.active.append(particle)
        # Gracefully handle pool exhaustion
        # Don't crash, just don't spawn
```

### LOD System for Enemies
```python
def update_enemy(enemy, player):
    distance = get_distance(enemy, player)
    
    if distance < 500:  # Close
        update_full_ai(enemy)
        update_animation(enemy)
        update_particles(enemy)
    elif distance < 1000:  # Medium
        update_simple_ai(enemy)
        update_animation(enemy, skip_frames=2)
        # No particles
    else:  # Far
        update_position_only(enemy)
        # No animation or particles
```

---

## Successful Onboarding Flow

### First Run Tutorial
```
Room 1: Movement only (no enemies)
- Teach: WASD/stick movement
- Reward: Speed boost pickup

Room 2: Single weak enemy
- Teach: Basic attack
- Reward: First boon choice (3 simple options)

Room 3: Three enemies
- Teach: Dash/dodge
- Reward: Heal pickup

Room 4: Elite enemy
- Teach: Telegraphs and patterns
- Reward: Rare boon

Room 5: Shop
- Teach: Economy and choices
- Guarantee: Enough gold for 1 purchase

Boss: Simple pattern
- Teach: Patience and observation
- Reward: Major power spike
```

---

## Endgame Loop Examples

### Daily Challenge Structure
```
Fixed Seed: Same for all players
Modifiers: 2-3 random each day
- "Enemies explode on death"
- "No shops"
- "Double boss HP"
Duration: 20-minute target
Leaderboard: Time and score
Reward: Cosmetic or currency
```

### Ascension System
```
Ascension 0: Base game
Ascension 1: Elites have +20% HP
Ascension 2: Shops cost 20% more
Ascension 3: Heal 50% less from all sources
...
Ascension 20: All modifiers active
```

Each ascension adds one modifier, creating natural progression and replayability.