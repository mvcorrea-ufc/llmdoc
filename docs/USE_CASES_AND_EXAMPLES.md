# LLMDocs - Use Cases and Practical Examples

## 🎯 Overview

This document provides real-world use cases showing how LLMDocs should intelligently process human descriptions and automatically create the appropriate documentation components. The system acts as an intelligent documentation assistant that understands context and creates structured data from natural language.

---

## 🚀 Use Case 1: Project Initialization from Description

### Human Input
```
"I'm starting a new e-commerce platform called 'ShopFlow'. It will have user authentication, product catalog with search, shopping cart, payment processing via Stripe, order management, and an admin dashboard. The tech stack will be React frontend, Node.js backend, PostgreSQL database, and Redis for caching. We plan to deploy on AWS using Docker containers."
```

### Expected System Behavior
The system should automatically create:

#### 1. Project Components
```bash
# System automatically executes:
llmdocs component add '{
  "id": "COMP-001",
  "name": "Frontend Application", 
  "type": "service",
  "description": "React-based user interface for e-commerce platform",
  "tech_stack": ["React", "JavaScript", "CSS", "HTML"],
  "dependencies": ["COMP-002"],
  "owner": "frontend-team"
}'

llmdocs component add '{
  "id": "COMP-002",
  "name": "Backend API",
  "type": "service", 
  "description": "Node.js REST API handling business logic",
  "tech_stack": ["Node.js", "Express", "JavaScript"],
  "dependencies": ["COMP-003", "COMP-004", "COMP-005"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-003",
  "name": "PostgreSQL Database",
  "type": "database",
  "description": "Primary data store for users, products, and orders",
  "tech_stack": ["PostgreSQL"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-004", 
  "name": "Redis Cache",
  "type": "external",
  "description": "Caching layer for session management and performance",
  "tech_stack": ["Redis"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-005",
  "name": "Stripe Payment Service",
  "type": "external",
  "description": "Third-party payment processing integration",
  "tech_stack": ["Stripe API"],
  "dependencies": [],
  "owner": "backend-team"
}'
```

#### 2. Initial Epic Tasks
```bash
# System creates high-level epics:
llmdocs task add '{
  "id": "TASK-001",
  "title": "User Authentication System",
  "type": "epic",
  "description": "Complete user registration, login, password reset functionality",
  "status": "todo",
  "priority": "high",
  "component_ids": ["COMP-002", "COMP-003"]
}'

llmdocs task add '{
  "id": "TASK-002", 
  "title": "Product Catalog and Search",
  "type": "epic",
  "description": "Product listing, filtering, search functionality with pagination",
  "status": "todo",
  "priority": "high",
  "component_ids": ["COMP-001", "COMP-002", "COMP-003"]
}'

llmdocs task add '{
  "id": "TASK-003",
  "title": "Shopping Cart Management", 
  "type": "epic",
  "description": "Add/remove items, cart persistence, quantity management",
  "status": "todo",
  "priority": "medium",
  "component_ids": ["COMP-001", "COMP-002", "COMP-004"]
}'

llmdocs task add '{
  "id": "TASK-004",
  "title": "Payment Processing Integration",
  "type": "epic", 
  "description": "Stripe integration for secure payment handling",
  "status": "todo",
  "priority": "high",
  "component_ids": ["COMP-002", "COMP-005"]
}'

llmdocs task add '{
  "id": "TASK-005",
  "title": "Order Management System",
  "type": "epic",
  "description": "Order tracking, status updates, history management", 
  "status": "todo",
  "priority": "medium",
  "component_ids": ["COMP-001", "COMP-002", "COMP-003"]
}'

llmdocs task add '{
  "id": "TASK-006",
  "title": "Admin Dashboard",
  "type": "epic",
  "description": "Administrative interface for managing products, orders, users",
  "status": "todo", 
  "priority": "low",
  "component_ids": ["COMP-001", "COMP-002"]
}'
```

#### 3. Initial Architecture Decisions
```bash
# System creates foundational ADRs:
llmdocs adr add '{
  "id": "ADR-001",
  "title": "Use React for Frontend Framework",
  "status": "accepted",
  "context": "Need modern, component-based UI framework for e-commerce platform",
  "decision": "Adopt React for frontend development with modern hooks and functional components",
  "consequences": "Pros: Large ecosystem, good performance, team familiarity. Cons: Learning curve for new developers"
}'

llmdocs adr add '{
  "id": "ADR-002", 
  "title": "Use PostgreSQL for Primary Database",
  "status": "accepted",
  "context": "Need reliable, ACID-compliant database for e-commerce transactions",
  "decision": "Use PostgreSQL as primary database for all transactional data",
  "consequences": "Pros: ACID compliance, JSON support, mature ecosystem. Cons: More complex than NoSQL for simple queries"
}'

llmdocs adr add '{
  "id": "ADR-003",
  "title": "Use Stripe for Payment Processing", 
  "status": "accepted",
  "context": "Need secure, PCI-compliant payment processing solution",
  "decision": "Integrate Stripe for all payment processing to ensure security and compliance",
  "consequences": "Pros: PCI compliance handled, excellent API. Cons: Transaction fees, vendor lock-in"
}'
```

---

## 🛠 Use Case 2: Feature Request Processing

### Human Input
```
"We need to add a product recommendation system to the e-commerce platform. It should analyze user behavior, purchase history, and similar user patterns to suggest relevant products. The system should show recommendations on the product detail page, cart page, and after checkout. We'll need to track user interactions, implement collaborative filtering algorithms, and ensure recommendations update in real-time. This should be completed in the next sprint."
```

### Expected System Behavior

#### 1. Analyze and Create Tasks
```bash
# System breaks down the feature into manageable tasks:
llmdocs task add '{
  "id": "TASK-007",
  "title": "Product Recommendation System",
  "type": "epic", 
  "description": "AI-powered product recommendations based on user behavior and purchase patterns",
  "status": "todo",
  "priority": "high",
  "component_ids": ["COMP-002", "COMP-003", "COMP-004"]
}'

# Sub-tasks automatically created:
llmdocs task add '{
  "id": "TASK-008",
  "title": "User Behavior Tracking",
  "type": "feature",
  "description": "Implement tracking for page views, clicks, time spent, cart additions",
  "status": "todo", 
  "priority": "high",
  "parent_id": "TASK-007",
  "story_points": 5,
  "component_ids": ["COMP-001", "COMP-002"]
}'

llmdocs task add '{
  "id": "TASK-009",
  "title": "Recommendation Algorithm Implementation",
  "type": "feature",
  "description": "Develop collaborative filtering algorithm for product suggestions", 
  "status": "todo",
  "priority": "high", 
  "parent_id": "TASK-007",
  "story_points": 8,
  "component_ids": ["COMP-002"]
}'

llmdocs task add '{
  "id": "TASK-010",
  "title": "Real-time Recommendation API",
  "type": "feature",
  "description": "Create API endpoints for fetching personalized recommendations",
  "status": "todo",
  "priority": "high",
  "parent_id": "TASK-007", 
  "story_points": 5,
  "component_ids": ["COMP-002", "COMP-004"]
}'

llmdocs task add '{
  "id": "TASK-011",
  "title": "Frontend Recommendation Components",
  "type": "feature",
  "description": "Build React components to display recommendations on product, cart, and checkout pages",
  "status": "todo",
  "priority": "medium",
  "parent_id": "TASK-007",
  "story_points": 8,
  "component_ids": ["COMP-001"]
}'
```

#### 2. Add to Current Sprint
```bash
# System checks current sprint and adds tasks:
CURRENT_SPRINT=$(llmdocs sprint current | jq -r '.id')

llmdocs task update TASK-008 --sprint $CURRENT_SPRINT
llmdocs task update TASK-009 --sprint $CURRENT_SPRINT  
llmdocs task update TASK-010 --sprint $CURRENT_SPRINT
llmdocs task update TASK-011 --sprint $CURRENT_SPRINT
```

#### 3. Create Relevant ADR
```bash
# System creates architectural decision:
llmdocs adr add '{
  "id": "ADR-004",
  "title": "Use Collaborative Filtering for Product Recommendations",
  "status": "proposed",
  "context": "Need to implement product recommendation system to increase sales and user engagement",
  "decision": "Implement collaborative filtering algorithm using user behavior patterns and purchase history",
  "consequences": "Pros: Personalized recommendations, proven approach. Cons: Cold start problem, computational complexity",
  "alternatives": ["Content-based filtering", "Hybrid approach", "Third-party service like AWS Personalize"]
}'
```

#### 4. Create User Stories
```bash
# System generates user stories from feature description:
llmdocs story add '{
  "id": "STORY-001", 
  "title": "Product Recommendations on Detail Page",
  "description": "As a customer browsing a product, I want to see related product recommendations so that I can discover items I might be interested in purchasing",
  "acceptance_criteria": [
    "Recommendations appear below product details",
    "Shows 4-6 relevant products", 
    "Recommendations update based on user behavior",
    "Clicking recommendation navigates to product page"
  ],
  "priority": "high",
  "story_points": 3
}'

llmdocs story add '{
  "id": "STORY-002",
  "title": "Cart Page Recommendations", 
  "description": "As a customer reviewing my cart, I want to see complementary product suggestions so that I can add related items to my purchase",
  "acceptance_criteria": [
    "Recommendations shown in cart sidebar",
    "Products complement items already in cart",
    "Easy one-click add to cart functionality",
    "Maximum 3 recommendations to avoid clutter"
  ],
  "priority": "medium",
  "story_points": 5
}'
```

---

## 🔄 Use Case 3: Bug Report Processing

### Human Input
```
"Users are reporting that the shopping cart loses items when they navigate away from the page and come back. This seems to happen especially on mobile devices. The cart should persist items across browser sessions and page refreshes. This is causing customers to abandon their purchases."
```

### Expected System Behavior

#### 1. Create Bug Task
```bash
# System automatically creates bug with appropriate details:
llmdocs task add '{
  "id": "TASK-012",
  "title": "Shopping Cart Items Lost on Navigation",
  "type": "bug",
  "description": "Cart items disappear when users navigate away and return, particularly on mobile devices causing purchase abandonment",
  "status": "todo",
  "priority": "critical",
  "component_ids": ["COMP-001", "COMP-004"],
  "labels": ["mobile", "cart", "persistence", "critical-bug"],
  "acceptance_criteria": [
    "Cart items persist across page navigation",
    "Cart items persist across browser sessions", 
    "Cart persistence works on all mobile devices",
    "No performance impact on cart operations"
  ]
}'
```

#### 2. Link to Existing Epic
```bash
# System intelligently links to shopping cart epic:
llmdocs task update TASK-012 --parent-id TASK-003
```

#### 3. Create Immediate Investigation Task
```bash
# System creates investigation task:
llmdocs task add '{
  "id": "TASK-013",
  "title": "Investigate Cart Persistence Issue",
  "type": "spike", 
  "description": "Research root cause of cart item loss on mobile devices and page navigation",
  "status": "todo",
  "priority": "critical",
  "parent_id": "TASK-012",
  "story_points": 2,
  "component_ids": ["COMP-001", "COMP-004"]
}'
```

---

## 🎨 Use Case 4: Technical Specification Processing

### Human Input
```
"We need to implement real-time notifications for order status updates. When an order status changes (placed, confirmed, shipped, delivered), customers should receive immediate notifications via email and push notifications. The system should support notification preferences, allowing users to choose which events they want to be notified about and through which channels. We should use WebSockets for real-time updates and integrate with SendGrid for emails and Firebase for push notifications."
```

### Expected System Behavior

#### 1. Create New Components
```bash
# System identifies new infrastructure needs:
llmdocs component add '{
  "id": "COMP-006",
  "name": "Notification Service",
  "type": "service",
  "description": "Real-time notification system for order status updates",
  "tech_stack": ["Node.js", "WebSockets", "Redis"],
  "dependencies": ["COMP-002", "COMP-004", "COMP-007", "COMP-008"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-007", 
  "name": "SendGrid Email Service",
  "type": "external",
  "description": "Email delivery service for transactional notifications",
  "tech_stack": ["SendGrid API"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-008",
  "name": "Firebase Push Notifications", 
  "type": "external",
  "description": "Mobile and web push notification delivery",
  "tech_stack": ["Firebase Cloud Messaging"],
  "owner": "backend-team"
}'
```

#### 2. Create Feature Epic and Tasks
```bash
# System breaks down complex feature:
llmdocs task add '{
  "id": "TASK-014",
  "title": "Real-time Order Notification System",
  "type": "epic",
  "description": "Complete notification system for order status updates via multiple channels",
  "status": "todo", 
  "priority": "high",
  "component_ids": ["COMP-006", "COMP-007", "COMP-008"]
}'

# Detailed sub-tasks:
llmdocs task add '{
  "id": "TASK-015",
  "title": "WebSocket Infrastructure Setup",
  "type": "task",
  "description": "Implement WebSocket server for real-time client connections",
  "status": "todo",
  "priority": "high",
  "parent_id": "TASK-014",
  "story_points": 5,
  "component_ids": ["COMP-006"]
}'

llmdocs task add '{
  "id": "TASK-016", 
  "title": "SendGrid Email Integration",
  "type": "task",
  "description": "Integrate SendGrid API for transactional email notifications",
  "status": "todo",
  "priority": "high", 
  "parent_id": "TASK-014",
  "story_points": 3,
  "component_ids": ["COMP-006", "COMP-007"]
}'

llmdocs task add '{
  "id": "TASK-017",
  "title": "Firebase Push Notification Setup",
  "type": "task", 
  "description": "Implement Firebase Cloud Messaging for push notifications",
  "status": "todo",
  "priority": "high",
  "parent_id": "TASK-014", 
  "story_points": 5,
  "component_ids": ["COMP-006", "COMP-008"]
}'

llmdocs task add '{
  "id": "TASK-018",
  "title": "User Notification Preferences",
  "type": "feature",
  "description": "Build interface for users to manage notification preferences", 
  "status": "todo",
  "priority": "medium",
  "parent_id": "TASK-014",
  "story_points": 8,
  "component_ids": ["COMP-001", "COMP-002"]
}'
```

#### 3. Create Technical ADRs
```bash
# System creates multiple related architectural decisions:
llmdocs adr add '{
  "id": "ADR-005",
  "title": "Use WebSockets for Real-time Notifications",
  "status": "proposed", 
  "context": "Need real-time communication for order status updates to provide immediate customer feedback",
  "decision": "Implement WebSocket connections for real-time notification delivery to connected clients",
  "consequences": "Pros: True real-time updates, low latency. Cons: Connection management complexity, server resource usage"
}'

llmdocs adr add '{
  "id": "ADR-006",
  "title": "Use SendGrid for Email Notifications",
  "status": "proposed",
  "context": "Need reliable email delivery service for transactional notifications",
  "decision": "Integrate SendGrid API for all transactional email notifications",
  "consequences": "Pros: High deliverability, analytics, template management. Cons: Service dependency, costs"
}'

llmdocs adr add '{
  "id": "ADR-007",
  "title": "Use Firebase for Push Notifications",
  "status": "proposed",
  "context": "Need cross-platform push notification solution for mobile and web clients", 
  "decision": "Implement Firebase Cloud Messaging for push notification delivery",
  "consequences": "Pros: Cross-platform support, reliable delivery. Cons: Google service dependency, setup complexity"
}'
```

---

## 🔄 Use Case 5: Project Evolution and Scaling

### Human Input
```
"The e-commerce platform is growing rapidly. We're now handling 10,000+ orders per day and need to scale our architecture. We should migrate to a microservices architecture, implement API rate limiting, add database read replicas, implement caching strategies, and consider moving to Kubernetes for orchestration. We also need to add monitoring and observability tools."
```

### Expected System Behavior

#### 1. Create Architecture Evolution Epic
```bash
# System creates scaling initiative:
llmdocs task add '{
  "id": "TASK-019",
  "title": "Platform Scaling and Microservices Migration", 
  "type": "epic",
  "description": "Architectural transformation to handle increased load and improve scalability",
  "status": "todo",
  "priority": "critical",
  "labels": ["scaling", "architecture", "performance", "microservices"]
}'
```

#### 2. Update Existing Components
```bash
# System evolves existing architecture:
llmdocs component add '{
  "id": "COMP-009",
  "name": "User Service",
  "type": "service", 
  "description": "Microservice handling user authentication and profile management",
  "tech_stack": ["Node.js", "Express", "JWT"],
  "dependencies": ["COMP-003"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-010",
  "name": "Product Service",
  "type": "service",
  "description": "Microservice managing product catalog and search functionality", 
  "tech_stack": ["Node.js", "Express", "Elasticsearch"],
  "dependencies": ["COMP-003", "COMP-011"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-011",
  "name": "Elasticsearch Search Engine",
  "type": "external",
  "description": "Search engine for product catalog and recommendations",
  "tech_stack": ["Elasticsearch"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-012",
  "name": "API Gateway", 
  "type": "service",
  "description": "Gateway handling routing, rate limiting, and authentication",
  "tech_stack": ["Kong", "Nginx"],
  "dependencies": ["COMP-009", "COMP-010", "COMP-013"],
  "owner": "devops-team"
}'

llmdocs component add '{
  "id": "COMP-013",
  "name": "Order Service",
  "type": "service",
  "description": "Microservice managing order processing and fulfillment",
  "tech_stack": ["Node.js", "Express"],
  "dependencies": ["COMP-003", "COMP-014"],
  "owner": "backend-team"
}'

llmdocs component add '{
  "id": "COMP-014",
  "name": "PostgreSQL Read Replicas",
  "type": "database", 
  "description": "Read-only database replicas for improved query performance",
  "tech_stack": ["PostgreSQL"],
  "dependencies": ["COMP-003"],
  "owner": "devops-team"
}'
```

#### 3. Create Infrastructure ADRs
```bash
# System creates architectural decisions for scaling:
llmdocs adr add '{
  "id": "ADR-008",
  "title": "Migrate to Microservices Architecture",
  "status": "proposed",
  "context": "Monolithic architecture cannot handle current scale of 10,000+ daily orders efficiently",
  "decision": "Break down monolith into domain-specific microservices for better scalability and maintainability", 
  "consequences": "Pros: Better scalability, team autonomy, technology diversity. Cons: Increased complexity, network latency, distributed system challenges",
  "alternatives": ["Vertical scaling", "Modular monolith", "Serverless functions"]
}'

llmdocs adr add '{
  "id": "ADR-009",
  "title": "Implement Database Read Replicas",
  "status": "proposed",
  "context": "Database read queries are becoming a bottleneck with increased traffic",
  "decision": "Set up PostgreSQL read replicas to distribute read load and improve performance",
  "consequences": "Pros: Improved read performance, reduced main DB load. Cons: Read lag, complexity in read/write separation"
}'

llmdocs adr add '{
  "id": "ADR-010", 
  "title": "Use Kubernetes for Container Orchestration",
  "status": "proposed",
  "context": "Need reliable container orchestration for microservices deployment and scaling",
  "decision": "Migrate to Kubernetes for container orchestration and auto-scaling",
  "consequences": "Pros: Auto-scaling, service discovery, rolling deployments. Cons: Learning curve, operational complexity"
}'
```

---

## 🎯 Use Case 6: Performance Issue Investigation

### Human Input
```
"The search functionality is very slow, taking 3-5 seconds to return results. Users are complaining about the poor search experience. We need to investigate database query performance, implement search indexing, possibly add caching, and consider alternative search solutions like Elasticsearch. This is affecting user engagement metrics significantly."
```

### Expected System Behavior

#### 1. Create Investigation Tasks
```bash
# System creates structured investigation:
llmdocs task add '{
  "id": "TASK-020",
  "title": "Search Performance Optimization",
  "type": "epic",
  "description": "Improve search functionality performance from 3-5s to under 500ms response time",
  "status": "todo",
  "priority": "critical",
  "component_ids": ["COMP-010", "COMP-003"],
  "labels": ["performance", "search", "critical"]
}'

llmdocs task add '{
  "id": "TASK-021",
  "title": "Database Query Performance Analysis",
  "type": "spike",
  "description": "Analyze slow search queries and identify performance bottlenecks", 
  "status": "todo",
  "priority": "critical",
  "parent_id": "TASK-020",
  "story_points": 3,
  "component_ids": ["COMP-003"]
}'

llmdocs task add '{
  "id": "TASK-022",
  "title": "Implement Database Search Indexes",
  "type": "task",
  "description": "Add appropriate indexes to product table for search queries",
  "status": "todo", 
  "priority": "high",
  "parent_id": "TASK-020",
  "story_points": 2,
  "component_ids": ["COMP-003"]
}'

llmdocs task add '{
  "id": "TASK-023",
  "title": "Evaluate Elasticsearch Integration",
  "type": "spike",
  "description": "Research and prototype Elasticsearch for product search functionality",
  "status": "todo",
  "priority": "high", 
  "parent_id": "TASK-020",
  "story_points": 5,
  "component_ids": ["COMP-010", "COMP-011"]
}'
```

#### 2. Link to Performance Metrics
```bash
# System creates performance tracking task:
llmdocs task add '{
  "id": "TASK-024", 
  "title": "Implement Search Performance Monitoring",
  "type": "task",
  "description": "Add metrics tracking for search response times and user engagement",
  "status": "todo",
  "priority": "medium",
  "parent_id": "TASK-020",
  "story_points": 3,
  "component_ids": ["COMP-010"]
}'
```

---

## 🔧 System Intelligence Features

### Automatic Context Understanding
The system should demonstrate intelligence by:

1. **Component Recognition**: Identifying which components are affected by a description
2. **Task Hierarchy**: Creating appropriate parent-child relationships
3. **Priority Assessment**: Setting priorities based on impact keywords (critical, slow, user complaints)
4. **Sprint Planning**: Automatically adding urgent items to current sprint
5. **Cross-Reference**: Linking related ADRs, tasks, and components
6. **Story Point Estimation**: Using historical data and complexity indicators

### Natural Language Processing Capabilities
The system should extract:

- **Technical Requirements**: APIs, databases, frameworks
- **Business Logic**: User workflows, business rules
- **Quality Attributes**: Performance, security, usability requirements
- **Dependencies**: Component relationships and integration points
- **Urgency Indicators**: Critical, urgent, affecting users, business impact

### Intelligent Categorization
Based on input patterns, the system should:

- **Epic vs Task**: Long descriptions with multiple components → Epic
- **Bug vs Feature**: Problem descriptions → Bug, new functionality → Feature
- **Spike vs Task**: Investigation/research keywords → Spike
- **ADR Triggers**: Architecture, technology choice, scalability → ADR needed

---

## 🎬 Example Interaction Flow

### Human Says:
```
"The mobile app crashes when users try to upload profile pictures larger than 2MB. We should implement image compression and validation before upload, and also add a loading indicator during the upload process."
```

### System Processes:
1. **Identifies**: Bug (crashes), Feature (compression, validation, loading)
2. **Components**: Mobile app, image upload service, user profile
3. **Priority**: High (crashes affect users)
4. **Creates**: Bug task + Feature tasks + Investigation spike

### System Outputs:
```bash
# Main bug task
llmdocs task add '{
  "id": "TASK-025",
  "title": "Mobile App Crashes on Large Image Upload",
  "type": "bug",
  "description": "App crashes when users upload profile pictures larger than 2MB",
  "status": "todo",
  "priority": "high",
  "component_ids": ["COMP-001"],
  "labels": ["mobile", "crash", "upload", "images"]
}'

# Feature tasks for solution
llmdocs task add '{
  "id": "TASK-026", 
  "title": "Implement Image Compression Before Upload",
  "type": "feature",
  "description": "Add client-side image compression to reduce file sizes before upload",
  "status": "todo",
  "priority": "high",
  "parent_id": "TASK-025",
  "story_points": 5
}'

llmdocs task add '{
  "id": "TASK-027",
  "title": "Add Upload Progress Indicator",
  "type": "feature", 
  "description": "Show loading indicator and progress during image upload process",
  "status": "todo",
  "priority": "medium",
  "parent_id": "TASK-025",
  "story_points": 3
}'
```

This intelligent processing allows teams to focus on building while the system handles the documentation structure automatically.