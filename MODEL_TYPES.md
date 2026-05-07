# Model Types Explanations

When building AI systems, people often say “model” to mean everything — but modern AI stacks are actually made of many specialized components.

LLMs are only one piece.

A real AI architecture (especially something like your Merix system) usually contains:

* tokenizers
* embedders
* rerankers
* vector indexes
* memory systems
* planners
* reasoning systems
* tool routers
* parsers
* classifiers
* speech systems
* diffusion/image models
* etc.

Here’s a practical breakdown.

---

## 1. Tokenizers

A tokenizer converts raw text into tokens.

Example:

```text
"Hello world"
```

Might become:

```text
[15496, 995]
```

LLMs do NOT understand text directly.
They understand token IDs.

---

### What tokenizers actually do

They:

* split text into pieces
* map pieces to integers
* compress language efficiently
* define the model’s vocabulary

---

### Common tokenizer algorithms

| Type                     | Notes                      |
| ------------------------ | -------------------------- |
| BPE (Byte Pair Encoding) | GPT-style                  |
| SentencePiece            | LLaMA/T5                   |
| WordPiece                | BERT                       |
| Unigram                  | probabilistic segmentation |

---

### Important architectural fact

The tokenizer is tightly coupled to the model.

You CANNOT safely mix:

* random tokenizer
* random LLM

Unless explicitly compatible.

---

### Why tokenizers matter

They affect:

* context length
* inference speed
* memory usage
* cost
* multilingual support
* code handling quality

---

## 2. Embedders (Embedding Models)

Embedders convert data into vectors.

Example:

```text
"The cat sat on the mat"
```

→

```text
[0.182, -0.442, 0.991, ...]
```

These vectors represent semantic meaning.

---

### What embeddings are for

Embeddings power:

* semantic search
* RAG
* memory retrieval
* clustering
* recommendation systems
* similarity matching
* agent memory
* long-term memory
* knowledge graphs

---

### Key concept

LLMs generate text.

Embedders generate meaning-space coordinates.

Huge difference.

---

### Typical embedding dimensions

| Model Type | Dimensions |
| ---------- | ---------- |
| Small      | 384        |
| Medium     | 768        |
| Large      | 1536–4096+ |

Higher dimensions:

* usually more semantic richness
* more RAM/storage cost

---

### Embedding model families

#### General semantic embedding

Good for:

* RAG
* search
* memory

Examples:

* BGE
* E5
* GTE
* OpenAI embeddings

---

#### Code embeddings

Optimized for:

* source code
* AST semantics
* repositories
* API similarity

Examples:

* CodeBERT
* Voyage-code
* Jina code embeddings

---

#### Multimodal embeddings

Map:

* text
* images
* audio

Into SAME vector space.

Examples:

* CLIP
* SigLIP

---

## 3. Rerankers

Rerankers are often misunderstood.

A reranker:

* takes candidate search results
* scores them more accurately

---

### Why rerankers exist

Vector search is approximate.

Embedding similarity alone is often mediocre.

Rerankers improve relevance dramatically.

---

### Flow

Typical RAG pipeline:

```text
Query
 → Embed
 → Vector search
 → Top 50 chunks
 → Reranker
 → Best 5 chunks
 → LLM
```

---

### Rerankers are usually cross-encoders

Meaning:

* query + document processed together
* more expensive
* much more accurate

---

### Important insight

Good reranking is often MORE impactful than:

* bigger LLM
* bigger embedding model

For retrieval quality.

---

## 4. Vector Databases

These store embeddings efficiently.

Examples:

* Qdrant
* Weaviate
* Milvus
* pgvector
* LanceDB

---

### What they do

They enable:

* nearest-neighbor search
* semantic retrieval
* memory lookup

Instead of:

```text
exact text matching
```

You get:

```text
meaning similarity
```

---

### Core algorithms

| Algorithm | Purpose                 |
| --------- | ----------------------- |
| HNSW      | fast approximate search |
| IVF       | clustered search        |
| PQ        | vector compression      |
| Flat      | brute force exact       |

---

### 5. Chunkers

Chunkers split data into retrieval units.

Example:

* split PDFs into sections
* split code into functions
* split chats into memory windows

---

### Why chunking matters

Bad chunking destroys RAG quality.

This is one of the most underestimated problems in AI systems.

---

### Types of chunking

| Type           | Description              |
| -------------- | ------------------------ |
| Fixed-size     | naive token windows      |
| Semantic       | meaning-aware            |
| Recursive      | hierarchical splitting   |
| AST-aware      | code structure aware     |
| Markdown-aware | document structure aware |

---

## 6. Parsers

Parsers convert raw input into structured data.

Examples:

* PDF parsers
* HTML parsers
* AST parsers
* Markdown parsers
* OCR systems

---

### In AI systems

Parsers often precede embeddings.

Example:

```text
PDF
 → parser
 → chunks
 → embeddings
 → vector DB
```

---

## 7. RAG Systems

RAG = Retrieval Augmented Generation.

This is not one component.
It is a pipeline.

Usually:

```text
Query
 → embed
 → retrieve
 → rerank
 → inject context
 → LLM generation
```

---

## 8. Memory Systems

This is where your architecture gets interesting.

Memory systems are NOT just vector DBs.

Advanced memory systems include:

* episodic memory
* semantic memory
* procedural memory
* temporal memory
* graph memory
* working memory
* long-term memory
* reflective memory

---

### Common memory layers

| Layer             | Purpose              |
| ----------------- | -------------------- |
| Working memory    | current context      |
| Episodic memory   | events               |
| Semantic memory   | facts                |
| Procedural memory | skills/workflows     |
| Vector memory     | similarity retrieval |
| Graph memory      | relationships        |
| Temporal memory   | time-aware recall    |

---

## 9. Rerieval Routers / Query Routers

These decide:

* where a query should go

Example:

```text
"Find Rust bug"
```

Router decides:

* code DB
* docs DB
* memory DB
* internet search
* graph DB

---

## 10. Tool Calling Systems

These let models invoke:

* APIs
* functions
* code
* search
* filesystem
* browsers

---

### Components

Usually:

* tool schema
* planner
* argument parser
* validator
* execution layer
* reflection layer

---

## 11. Planners

Planners break tasks into steps.

Example:

```text
Goal:
"Deploy backend"

Plan:
1. Build
2. Run tests
3. Build docker image
4. Push
5. Deploy
```

---

### Important distinction

LLM ≠ planner.

LLMs can emulate planning.

But dedicated planning systems:

* maintain state
* validate dependencies
* manage execution graphs

---

## 12. Reasoning Systems

Separate from generation.

Can include:

* tree search
* graph traversal
* symbolic reasoning
* chain-of-thought
* sequential thinking
* Monte Carlo planning
* constraint solving

---

## 13. Classifiers

Small specialized models.

Used for:

* moderation
* intent detection
* routing
* sentiment
* spam
* priority estimation

Usually MUCH smaller than LLMs.

---

## 14. Diffusion Models

Image/video generation systems.

Not token predictors like LLMs.

They:

* denoise latent space iteratively

Examples:

* Stable Diffusion
* Flux
* SDXL

---

## 15. Speech Models

Usually split into:

| System           | Purpose             |
| ---------------- | ------------------- |
| STT              | speech-to-text      |
| TTS              | text-to-speech      |
| Voice cloning    | speaker replication |
| Audio embeddings | speaker recognition |

---

## 16. Attention Systems

This becomes important for large agent architectures.

Attention systems decide:

* what matters RIGHT NOW

Examples:

* context prioritization
* memory scoring
* relevance decay
* recency weighting
* salience tracking

---

## 17. Context Compression Systems

Needed because context windows are finite.

These:

* summarize
* compress
* distill
* prune memory/context

---

## 18. Fine-Tuning Systems

Ways to specialize models:

* LoRA
* QLoRA
* adapters
* full finetuning
* RLHF
* DPO

---

## 19. Quantization Systems

Reduce model size:

* INT8
* INT4
* GGUF
* AWQ
* GPTQ

Tradeoff:

* quality vs speed/RAM

---

## 20. Inference Engines

The runtime serving layer.

Examples:

* vLLM
* TensorRT-LLM
* llama.cpp
* TGI
* ONNX Runtime

These matter enormously for performance.

---

## The big realization

Modern AI systems are becoming:

```text
many specialized systems
```

NOT:

```text
one giant model
```

And honestly, architecture quality now matters more than raw model size in many real-world systems.

Especially for:

* agents
* memory systems
* autonomous workflows
* coding systems
* long-running AI environments

That’s why your crate separation direction actually makes sense:

* storage
* memory
* attention
* capabilities
* agents
* routing
* tools

That is much closer to where advanced AI architectures are heading than “single chatbot wrapper around GPT”.
