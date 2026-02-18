# Paso 1 - MVP (Hexagonal + Screaming)

1) Contexto y objetivo
- Proyecto: CLI To-Do en Rust
- Arquitectura objetivo: Hexagonal + Screaming Architecture
- Objetivo MVP: gestionar tareas desde terminal con persistencia local
- No objetivo (por ahora): sync en la nube, multiusuario, UI TUI compleja

2) Casos de uso del MVP (input ports)
   Define cada caso con intención + resultado.
- AddTask
    - Entrada: title
    - Regla: título no vacío
    - Resultado: tarea creada con id único y estado Todo
- ListTasks
    - Entrada: filtro opcional (all|todo|done)
    - Resultado: listado de tareas
- MarkTaskDone
    - Entrada: id
    - Regla: debe existir
    - Resultado: estado cambia a Done
- DeleteTask
    - Entrada: id
    - Regla: debe existir
    - Resultado: tarea eliminada
3) Modelo de dominio (borrador)
- Entidad Task
    - id: u64
    - title: String
    - status: Todo | Done
    - created_at: DateTime (opcional en MVP si quieres simplificar)
    - deadline: Option<Date> (futuro)
    - repeat: Option<RepeatRule> (futuro)
      Decisión inicial recomendada:
- IDs incrementales (u64) para no complicar UX del CLI al principio.
4) Reglas de negocio
- No se permite título vacío.
- id debe ser único.
- Marcar/eliminar una tarea inexistente devuelve error de dominio explícito.
- list sin filtro devuelve todas.
5) Diseño hexagonal (puertos y adaptadores)
- Input ports (driving):
    - TaskCommandService o casos de uso separados (AddTaskUseCase, etc.)
- Output ports (driven):
    - TaskRepository (guardar, obtener, buscar por id, borrar)
- Adapters:
    - Entrada: CLI adapter (parse args -> invoca casos de uso)
    - Salida: File JSON adapter (implementa TaskRepository)
6) Estructura screaming (acordada)
   src/
   tasks/
   domain/
   application/
   ports/
   adapters/
   cli/
   persistence/
   Notas:
- tasks es la raíz “que grita” el dominio.
- Evita src/core, src/utils genéricos al inicio (suelen esconder responsabilidades).
7) Contrato CLI (UX mínima)
   Define sintaxis esperada:
- todo add "Comprar leche"
- todo list
- todo list --status todo
- todo done 3
- todo delete 3
  Define también:
- Mensajes de éxito (1 línea)
- Mensajes de error (claros y accionables)
8) Persistencia (MVP)
- Archivo: .todo/tasks.json
- Si no existe: crear o tratar como vacío.
- Guardar después de operaciones mutantes (add/done/delete).
- Error de JSON inválido: mensaje claro + no perder datos silenciosamente.
9) Errores (taxonomía mínima)
- ValidationError (ej. title vacío)
- NotFound (id inexistente)
- PersistenceError (lectura/escritura/parsing)
- CliInputError (comando/argumento inválido)
10) Definition of Done (Paso 1)
    Marca ✅ cuando esté cerrado:
- [ ] Casos de uso MVP definidos
- [ ] Reglas de negocio mínimas definidas
- [ ] Puertos de entrada/salida identificados
- [ ] Adaptadores CLI + File definidos
- [ ] Estructura screaming acordada
- [ ] Contrato de comandos CLI documentado
- [ ] Estrategia de persistencia y errores definida