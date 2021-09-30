# Unit Orders & Decision Making

## Outline
Decision-making is at the core of Condottiere. Units are given missions by other units higher up, which all stems from the decision the player actuates. While the player controls the broader strokes of each unit's strategy, equipment, commanders, and goals, it doesn't have direct control over modifying and setting them, instead having to hope that the decisions and preparation they've done over earlier parts of the game pays off through an effective fighting force.

## The Process
--- Something about something ---

The process starts at the 'Strategic Command', which is where the player can set the grand objectives for the campaign, such as conquering regions or cities, holding onto strategic areas, or destruction of parts of the opposition's army, industry, or trade. There (should be) many objectives the player can choose from.

After this, each unit is assigned a `Mission`. A mission is the overarching goal that a unit will strive towards with all their available resources, that being political clout, officers, units, or otherwise. 

[comment]: <> (Units will be able to set their subordinate unit's `Mission`, and hence the controlling unit can shape the )

For non-player factions, his process is automated through a `System` which is responsible for determining the necessary objectives, as well as the necessary  resources for them as well. 

```

                    | 
                    | Set by their own superior unit
+---------------+   v
|               | - Mission: Their supreme goal (high level)
| Superior Unit | - Objective: The action the unit takes (low level)
|               | 
+---------------+
 | (1)
 |
 v (many)
+---------------+
|  Junior Unit  | - Mission
+---------------+
```

A `Mission` is a high level idea of what a unit (and all subordinate units) should be aiming to achieve. This is something more conceptual compared to a single hard goal. In achieving a `Mission`, some state of the world will have to be achieved, likely through numerous `Objectives`

An `Objective` should be an actual measurable difference in the game state, something like a unit moving, a unit shooting, or a unit preparing fortifications/ambushes/etc. There's probably many more that'll appear over time.

A list of `Missions` and `Objectives`

| Missions    | Objectives  |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |