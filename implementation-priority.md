# Roguelike ARPG Implementation Priority Guide

## Phase 1: Core Foundation (Week 1-2)
**Goal: Playable prototype with core loop**

### Critical Path (Do First)
1. **Movement System**
   - [ ] 8-directional movement
   - [ ] Dash/dodge with i-frames
   - [ ] Collision detection
   - [ ] Input buffering (100ms)
   
2. **Basic Combat**
   - [ ] Light attack combo (3 hits)
   - [ ] Heavy attack
   - [ ] Hit detection & hurtboxes
   - [ ] Damage numbers
   - [ ] Basic knockback

3. **Enemy AI v1**
   - [ ] Melee enemy (approach & attack)
   - [ ] Ranged enemy (maintain distance & shoot)
   - [ ] Basic telegraph system
   - [ ] Death & spawn system

4. **Room System**
   - [ ] Room loading/transitions
   - [ ] Door locking during combat
   - [ ] Basic arena layout
   - [ ] Victory condition

5. **Health System**
   - [ ] Player HP & death
   - [ ] Enemy HP bars
   - [ ] Damage calculation
   - [ ] Basic healing pickup

### Success Criteria
- Can complete 3 combat rooms in sequence
- Combat feels responsive (< 16ms input lag)
- Clear win/lose states
- 30-second core loop works

---

## Phase 2: Choice & Progression (Week 3-4)
**Goal: Meaningful decisions and power growth**

### Priority Features
1. **Boon System**
   - [ ] Boon data structure
   - [ ] 10 starter boons (5 offensive, 5 defensive)
   - [ ] Choice UI (3 options)
   - [ ] Boon application to stats
   - [ ] Visual feedback for active boons

2. **Power Scaling**
   - [ ] Damage formula with multipliers
   - [ ] Crit chance & damage
   - [ ] Attack speed modifiers
   - [ ] Stat UI display

3. **Room Rewards**
   - [ ] Post-combat reward screen
   - [ ] Gold drops & collection
   - [ ] Boon selection after room clear
   - [ ] Chest spawning

4. **Enemy Variety**
   - [ ] Elite enemies (2x HP, special attacks)
   - [ ] Support enemy (buffs others)
   - [ ] Tank enemy (shield mechanic)
   - [ ] Mixed encounter design

5. **Basic Shop**
   - [ ] Shop room type
   - [ ] Buy/sell UI
   - [ ] Item pricing
   - [ ] Heal option
   - [ ] Reroll mechanic

### Success Criteria
- 20+ minute runs possible
- Clear power progression felt
- Choices feel meaningful
- Build identity emerging

---

## Phase 3: First Boss & Biome (Week 5-6)
**Goal: Complete vertical slice of content**

### Priority Features
1. **Boss Fight**
   - [ ] Multi-phase boss (3 phases)
   - [ ] 5+ attack patterns
   - [ ] Phase transitions
   - [ ] Boss-specific arena
   - [ ] Epic reward

2. **Biome Structure**
   - [ ] 15-room progression
   - [ ] Room type variety (combat/shop/treasure)
   - [ ] Minimap system
   - [ ] Path choices (2-3 exits per room)

3. **Advanced Boons**
   - [ ] 30+ total boons
   - [ ] Synergy system (tags & combos)
   - [ ] Rarity tiers (common/rare/epic)
   - [ ] Visual effects for boons

4. **Risk/Reward Mechanics**
   - [ ] Elite rooms with bonus rewards
   - [ ] Curse system (-HP for +damage)
   - [ ] Timed challenges
   - [ ] Secret rooms (10% chance)

5. **Polish Pass 1**
   - [ ] Screen shake on impact
   - [ ] Particle effects for hits
   - [ ] Basic sound effects
   - [ ] Death animations

### Success Criteria
- Boss fight takes 3-5 minutes
- Complete runs possible
- 30-40 minute full runs
- Replayability evident

---

## Phase 4: Meta Progression (Week 7-8)
**Goal: Long-term player retention systems**

### Priority Features
1. **Persistent Progression**
   - [ ] Account level system
   - [ ] Currency that persists between runs
   - [ ] Unlock system for new content
   - [ ] Achievement tracking

2. **Starting Loadouts**
   - [ ] 3+ weapon types
   - [ ] Character selection
   - [ ] Starting boon choices
   - [ ] Difficulty modifiers

3. **Extended Content**
   - [ ] Second biome with new enemies
   - [ ] Second boss
   - [ ] 50+ total boons
   - [ ] Legendary tier items

4. **Quality of Life**
   - [ ] Save/continue run
   - [ ] Stats tracking
   - [ ] Build history
   - [ ] Settings menu (audio/video/controls)

5. **Replay Features**
   - [ ] Daily challenge mode
   - [ ] Seeded runs
   - [ ] Leaderboards
   - [ ] Speed run timer

### Success Criteria
- 10+ hours of content
- Clear meta progression
- Multiple viable builds
- Daily retention mechanics work

---

## Phase 5: Full Game Polish (Week 9-12)
**Goal: Ship-ready quality**

### Priority Features
1. **Complete Content**
   - [ ] 4+ biomes
   - [ ] 5+ bosses
   - [ ] 100+ boons/items
   - [ ] 10+ enemy types per biome

2. **Full Audio**
   - [ ] Dynamic music system
   - [ ] Combat sound effects
   - [ ] UI sounds
   - [ ] Voice barks (optional)

3. **Visual Polish**
   - [ ] Consistent art style
   - [ ] Smooth animations
   - [ ] Environmental effects
   - [ ] Lighting system

4. **Balancing**
   - [ ] Difficulty curve tuning
   - [ ] Economy balancing
   - [ ] Build viability testing
   - [ ] Exploit fixing

5. **Platform Features**
   - [ ] Cloud saves
   - [ ] Controller support
   - [ ] Multiple resolutions
   - [ ] Achievements/trophies

### Success Criteria
- No game-breaking bugs
- 60 FPS stable
- All systems integrated
- Ready for public release

---

## Technical Priority Stack

### Must Have (Week 1)
```
- Game loop
- Input system  
- Collision
- Basic rendering
- Entity system
```

### Should Have (Week 2-4)
```
- Save system
- Audio manager
- Particle system
- UI framework
- Procedural generation
```

### Nice to Have (Week 5+)
```
- Shader effects
- Post-processing
- Networking
- Mod support
- Replay system
```

---

## Team Resource Allocation

### Solo Developer
- Week 1-2: Core combat
- Week 3-4: Progression
- Week 5-8: Content creation
- Week 9-12: Polish & balance

### Small Team (3-5)
- **Programmer**: Core systems & gameplay
- **Designer**: Balance, content, levels
- **Artist**: Assets, animations, VFX
- **Optional**: Audio, QA

### Medium Team (10+)
- **Core Gameplay**: 2-3 programmers
- **Systems**: 2 programmers (meta, progression)
- **Content**: 2-3 designers
- **Art**: 2-3 artists
- **QA**: 1-2 testers
- **Producer**: 1 coordinator

---

## Risk Mitigation

### High Risk Areas
1. **Combat Feel** - Prototype early, iterate often
2. **Performance** - Profile from day 1
3. **Balance** - Automated testing, analytics
4. **Procedural Generation** - Have backup layouts
5. **Multiplayer** (if applicable) - Architect early

### Contingency Plans
- **Scope Creep**: Cut biomes, not systems
- **Performance Issues**: Reduce particle effects first
- **Balance Problems**: Add difficulty modes
- **Content Shortage**: Focus on replay value
- **Technical Debt**: Schedule refactor sprints

---

## Minimum Shippable Product

### MVP Checklist
- [ ] 1 complete biome (15 rooms)
- [ ] 1 boss fight
- [ ] 30 boons/items
- [ ] 5 enemy types
- [ ] Basic meta progression
- [ ] Shop system
- [ ] Win/lose conditions
- [ ] Basic audio/visual polish

### MVP Metrics
- 20-30 minute runs
- 5% win rate for new players
- 3+ viable build paths
- No critical bugs
- 60 FPS on target hardware

---

## Post-Launch Roadmap

### Month 1
- Bug fixes
- Balance patches
- Quality of life improvements

### Month 2-3
- New biome
- New boss
- 20+ new items
- New character/weapon

### Month 4-6
- Major content update
- New game mode
- Seasonal events
- Community features

### Year 1 Goals
- 2x content from launch
- 80% player retention (month 1)
- 4+ major updates
- Active community
- Positive reviews (>85%)

---

## Development Mantras

### Always Remember
1. **Fun First** - If it's not fun, fix it or cut it
2. **Feel Matters** - Polish combat before adding features
3. **Respect Time** - Fast restarts, clear progress
4. **Test Daily** - Play your game every day
5. **Ship It** - Perfect is the enemy of good

### Never Forget
- Players will find exploits - embrace them
- RNG needs boundaries - control the chaos
- Death should teach - make it fair
- Choices need weight - no false options
- Progress must be felt - show the power

---

## Quick Decision Framework

### When Adding Features, Ask:
1. Does it enhance the core loop?
2. Can players understand it in 5 seconds?
3. Does it create interesting decisions?
4. Will 80% of players engage with it?
5. Can we polish it properly?

**If 3+ Yes → Build it**
**If 2 Yes → Prototype it**
**If 1 Yes → Backlog it**
**If 0 Yes → Cut it**