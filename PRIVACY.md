# Privacy & Architectural Boundaries

Merix is being built as a **local-first AI operating system**. With that power comes serious responsibility.

A system with semantic memory, workflow awareness, long-term behavioral modeling, per-user profiles, agent orchestration, and deep contextual understanding can generate user intelligence profiles that are dramatically richer than anything produced by today’s browsers, phones, social networks, or search engines.

That same capability makes Merix potentially far more invasive than traditional ad-tech—if the architecture and governance are not designed correctly from day one.

## Capability vs. Deployment Model

Technically, yes: a malicious actor could build a highly exploitative profiling system on top of Merix’s primitives.

But there is a fundamental difference between:

- Building privacy-preserving local intelligence  
- Extracting, storing, or selling inferred psychological profiles

The second path becomes dangerous very quickly because **inference data is often more sensitive than raw data**.

Example:

- “User searched for stress symptoms” → relatively benign.
- “System predicts 82% probability of burnout + elevated impulsive purchasing behavior” → deeply invasive.

## Core Privacy Principles

**Capabilities should not automatically imply permission.**

Just because the system *can* infer something does not mean it should:

- store it
- expose it
- monetize it
- transmit it
- retain it indefinitely

### Hard Architectural Boundaries (non-negotiable)

1. **What never leaves the device**
   - Raw embeddings
   - Semantic graphs / memory entries
   - Documents or file contents
   - Prompts or conversation history
   - User intent chains or reasoning traces
   - Emotional inferences or psychological profiles

2. **Local-first matching only**
   - For monetization / recommendations (see MONETIZATION.md), the server sends only encrypted/tagged campaign metadata.
   - All semantic matching, scoring, and ranking happens locally using `merix-core::monetization` + existing memory/embeddings.
   - Only anonymous impression/click events (campaign ID, slot, clicked) are ever transmitted.

3. **Profile isolation**
   - Every profile (work, private journal, anonymous, etc.) maintains completely separate memory stores, embeddings, and ad/monetization policies.
   - “Private mode” must disable all persistence and monetization features.

4. **Explainability & auditability**
   - Every recommendation or contextual suggestion must be explainable on demand (“Why did this appear?”).
   - Users can inspect matching tags/categories and the local memory entries that contributed (without exposing raw data externally).

5. **No global agent memory access**
   - Agents and plugins operate within strict capability scopes and cannot read across profiles without explicit user consent.

6. **Data export & deletion**
   - Embeddings and semantic graphs are never exportable.
   - Full user-controlled deletion of any profile/memory must be one-click and irreversible.

## Relation to Monetization

The “local-first AdSense” model described in MONETIZATION.md is only acceptable **because** of these boundaries.  
It is not surveillance advertising—it is local semantic recommendation that respects the above rules.  
If any part of the implementation ever violates these boundaries, the monetization feature must be disabled or redesigned.

## Governance & Future-Proofing

- All privacy boundaries must be enforced at the architectural level (not just in UI or policy).
- Future contributors and third-party capability developers must be able to verify these guarantees through code and documentation.
- Transparency is a competitive advantage: the more powerful Merix becomes, the more valuable a genuinely private, inspectable, user-controlled system will be.

Merix should be the AI OS that people trust precisely because it is designed so that even its creators cannot spy on users.

This document is living. It will be updated as the implementation matures, but the core principles above are foundational and will not change.
