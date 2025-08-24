# Roguelike ARPG Designer Checklist

## Pre-Production Checklist

### Core Loop Validation
- [ ] Can player complete core loop in under 30 seconds?
- [ ] Does each step feel distinct and necessary?
- [ ] Is there a clear fail state that feels fair?
- [ ] Can player restart within 3 seconds?

### Build Identity
- [ ] Define 5 distinct archetypes
- [ ] Each archetype has 3+ win conditions
- [ ] Clear counter-play for each archetype
- [ ] Synergies are discoverable, not hidden

---

## Room Design Checklist

### Every Room Must Have
- [ ] Clear entry/exit points
- [ ] Visual landmark for orientation
- [ ] 3-second grace period on entry
- [ ] Reward that matches risk level

### Room Variety (Per Biome)
- [ ] 40% Arena rooms (combat focus)
- [ ] 40% Gauntlet rooms (traversal focus)  
- [ ] 20% Puzzle/Secret rooms
- [ ] 10-20% contain secrets

### Pacing Per Run
- [ ] Room 1-5: Tutorial pace (30s each)
- [ ] Room 6-15: Standard pace (45s each)
- [ ] Room 16-20: Intense pace (60s each)
- [ ] Boss: 3-5 minutes

---

## Combat Checklist

### Input Requirements
- [ ] Input lag < 16ms
- [ ] 100ms input buffer for combos
- [ ] Cancel windows on 80% of animations
- [ ] Dash always responsive

### Telegraph Standards
- [ ] Minimum 0.5s warning
- [ ] Maximum 2s telegraph
- [ ] Color coded by type (Red=damage, Yellow=stun, Purple=debuff)
- [ ] Audio cue precedes visual

### Enemy Mix Per Room
- [ ] 1 role minimum (damage OR control OR support)
- [ ] 2 roles standard
- [ ] 3 roles for elite rooms
- [ ] Never more than 12 enemies active

---

## Choice Design Checklist

### Every Choice Screen
- [ ] Exactly 3 options
- [ ] One safe, one risky, one weird
- [ ] Clear tradeoffs visible
- [ ] Synergy icons if applicable
- [ ] Can compare side-by-side

### Choice Distribution
- [ ] 75% enhance current build
- [ ] 25% offer pivot opportunity
- [ ] Never 3 of same type
- [ ] Never all offensive or all defensive

### Information Shown
- [ ] Exact numbers, not percentages
- [ ] Damage ranges if variable
- [ ] Cooldowns and durations
- [ ] Stack limits if applicable

---

## Power Progression Checklist

### Breakpoints to Hit
- [ ] Room 5: One-shot basic enemies
- [ ] Room 10: Two-shot elite enemies
- [ ] Room 15: Screen-clear potential
- [ ] Room 20: Build fully online

### Power Sources
- [ ] 3 active abilities max
- [ ] 6 passive boons max
- [ ] 1 curse slot (optional risk)
- [ ] 3 equipment slots

### Feel Milestones
- [ ] First power-up: New verb
- [ ] Second power-up: Stat threshold crossed
- [ ] Third power-up: Synergy activated
- [ ] Fourth+ power-up: Exponential scaling

---

## Risk/Reward Checklist

### Risk Options Per Biome
- [ ] Elite room (2x reward, 3x enemy HP)
- [ ] Curse shrine (-20% HP, +40% damage)
- [ ] Timed challenge (30s for 3x loot)
- [ ] Blood door (25% current HP cost)

### Escape Valves
- [ ] Low risk: Always has escape
- [ ] Medium risk: Costs resources to escape
- [ ] High risk: One-time escape item
- [ ] Extreme risk: No escape

### Clear Communication
- [ ] Show exact costs upfront
- [ ] Preview rewards before commitment
- [ ] Success conditions explicit
- [ ] Failure consequences clear

---

## Economy Checklist

### Currency Balance
- [ ] Gold: Abundant but never enough
- [ ] Souls: Rare, 1-3 per run
- [ ] HP: Tradeable resource

### Shop Must-Haves
- [ ] Heal option (small always available)
- [ ] Remove option (bad boon removal)
- [ ] Reroll option (increasing cost)
- [ ] One "god tier" expensive item

### Pricing Guidelines
- [ ] Common: 1x room reward
- [ ] Rare: 3x room reward
- [ ] Epic: 8x room reward
- [ ] Legendary: All current gold

---

## Boss Fight Checklist

### Phase Structure
- [ ] Phase 1 (50% HP): Teaching
- [ ] Phase 2 (30% HP): Combination
- [ ] Phase 3 (20% HP): Desperation

### Fairness Rules
- [ ] All attacks telegraphed in phase 1
- [ ] No new attacks after 50% HP
- [ ] 2-second breathing room between phases
- [ ] Checkpoints at phase transitions

### Rewards
- [ ] Guaranteed epic item
- [ ] Full heal
- [ ] Meta currency
- [ ] Choice of 3 legendaries

---

## Meta Progression Checklist

### Unlock Order
1. [ ] Horizontal variety (weapons, characters)
2. [ ] Quality of life (starting items, shortcuts)
3. [ ] Power increases (stats, bonuses)

### Pacing
- [ ] First unlock: 1-2 runs
- [ ] Major unlock: Every 5 runs
- [ ] Full unlocks: 50-100 runs
- [ ] Mastery content: 200+ runs

### Daily Challenges
- [ ] 15-20 minute runs
- [ ] Fixed seed
- [ ] One unique modifier
- [ ] Leaderboard with ghosts

---

## Polish Checklist

### Visual Feedback
- [ ] Damage numbers (white/yellow/red)
- [ ] Hit effects match weapon type
- [ ] Screen shake scales with impact
- [ ] Death effects are satisfying

### Audio Feedback  
- [ ] Hit confirm sound
- [ ] Crit has unique sound
- [ ] Low HP warning
- [ ] Secret discovery chime

### UI/UX
- [ ] One button restart
- [ ] Build summary screen
- [ ] Damage dealt/taken log
- [ ] Seed visible for sharing

---

## Performance Targets

### Frame Rate
- [ ] 60 FPS minimum
- [ ] 120 FPS target
- [ ] No drops below 30 FPS
- [ ] Particle limits in place

### Load Times
- [ ] Game start: < 10 seconds
- [ ] Run restart: < 3 seconds
- [ ] Room transition: < 1 second
- [ ] Shop load: Instant

### Input Latency
- [ ] Controller: < 16ms
- [ ] Keyboard: < 8ms
- [ ] Network (if applicable): < 100ms

---

## Playtesting Checklist

### New Player Experience
- [ ] Can complete first room without dying
- [ ] Understands all 3 resources by room 5
- [ ] Makes first build choice confidently
- [ ] Dies with clear understanding of why

### Experienced Player Targets
- [ ] Win rate: 20-40%
- [ ] Average run: 25-35 minutes
- [ ] Build variety: Uses 80% of items
- [ ] Still discovering synergies at 50 hours

### Feedback to Gather
- [ ] Death feels fair? (Yes/No)
- [ ] Want to play again? (1-10)
- [ ] Favorite moment?
- [ ] Most frustrating part?
- [ ] Build preference?

---

## Red Flags to Avoid

### Never Ship With
- ❌ Off-screen deaths
- ❌ Unavoidable damage
- ❌ Builds that can't win
- ❌ Infinite stun locks
- ❌ Hidden mechanics
- ❌ Pay-to-win elements
- ❌ Forced tutorials after first play
- ❌ RNG that breaks runs

### Always Ship With
- ✅ Clear death recap
- ✅ Instant restart
- ✅ Build variety
- ✅ Fair difficulty curve
- ✅ Visible progress
- ✅ Satisfying audio/visual feedback
- ✅ Respect for player time
- ✅ "One more run" feeling