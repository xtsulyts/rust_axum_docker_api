// Importamos los módulos necesarios de Axum para crear rutas HTTP
// routing contiene las funciones para definir rutas (get, post)
// Router es el struct principal para construir nuestro enrutador
use axum::{
    routing::{get, post},  // Importamos específicamente get y post del módulo routing
    Router,                 // Importamos el tipo Router
};

// Importamos tipos de la biblioteca estándar de Rust para concurrencia
use std::sync::Arc;     // Arc = Atomic Reference Counter (permite compartir datos entre hilos)
use std::sync::Mutex;   // Mutex = Mutual Exclusion (protege datos contra acceso simultáneo)

// Importamos nuestros módulos locales (crate = nuestro proyecto)
use crate::handlers::user_handler::{get_users, get_user_by_id, create_user};  // Los manejadores de las rutas
use crate::models::user::User;  // El modelo/estructura de usuario

// Definimos un tipo personalizado para nuestra "base de datos" en memoria
// Arc<Mutex<HashMap<u32, User>>> significa:
// - HashMap: almacena usuarios con ID como clave (u32) y User como valor
// - Mutex: permite que solo un hilo modifique el HashMap a la vez
// - Arc: permite compartir el Mutex entre múltiples hilos de forma segura
type UserDb = Arc<Mutex<std::collections::HashMap<u32, User>>>;

// Función pública que configura todas las rutas relacionadas con usuarios
// Recibe la base de datos (UserDb) como parámetro
// Retorna un Router configurado con las rutas de usuarios
pub fn users_routes(db: UserDb) -> Router {
    // Creamos un nuevo Router y encadenamos las rutas usando .route()
    Router::new()
        // Ruta GET para "/" (lista todos los usuarios)
        .route("/", get({
            // Clonamos la referencia a la base de datos para moverla al closure
            // Esto incrementa el contador de referencias de Arc
            let db = db.clone();
            
            // Closure que captura 'db' y llama a get_users
            // El 'move' transfiere la propiedad de 'db' al closure
            // Nota: get_users espera recibir la db como parámetro
            move || get_users(db.clone())  // Clonamos db para pasarla a get_users
        }))
        
        // Ruta POST para "/" (crea un nuevo usuario)
        .route("/", post({
            // De nuevo clonamos db para cada closure
            let db = db.clone();
            
            // Closure que recibe un payload (los datos del usuario a crear)
            // El payload viene del cuerpo de la petición HTTP
            move |payload| create_user(payload, db.clone())  // Pasamos payload y db a create_user
        }))
        
        // Ruta GET para "/{id}" (obtiene un usuario por su ID)
        .route("/{id}", get({
            let db = db.clone();
            
            // Closure que recibe el 'path' (contiene el parámetro {id} de la URL)
            move |path| get_user_by_id(path, db.clone())  // Pasamos path y db a get_user_by_id
        }))
}

/* 
EXPLICACIÓN DETALLADA PARA PRINCIPIANTES:

1. **Closures y 'move'**:
   - Los closures son funciones anónimas que pueden capturar variables del entorno
   - 'move' fuerza al closure a tomar propiedad de las variables que captura
   - Esto es necesario porque las rutas de Axum se ejecutan en diferentes hilos

2. **¿Por qué tantos clones?**:
   - Cada ruta necesita su propia copia de la referencia a la base de datos
   - db es un Arc, clonarlo solo incrementa un contador, no copia los datos
   - Es económico y necesario para compartir la DB entre rutas

3. **Arc<Mutex<T>> explicado**:
   - Arc permite que múltiples hilos tengan propiedad compartida de T
   - Mutex garantiza que solo un hilo pueda modificar T a la vez
   - Juntos permiten compartir datos de forma segura entre hilos

4. **Flujo de las peticiones**:
   - Cuando llega una petición HTTP, Axum ejecuta el closure correspondiente
   - El closure llama al handler (get_users, create_user, etc.)
   - Los handlers reciben la DB y otros parámetros, y devuelven una respuesta

5. **Nota sobre parámetros**:
   - El closure de GET / no recibe parámetros
   - El closure de POST / recibe 'payload' (datos JSON del cuerpo)
   - El closure de GET /{id} recibe 'path' (contiene el ID de la URL)

6. **Type UserDb**:
   - Es un alias para no tener que escribir Arc<Mutex<HashMap<u32, User>>> cada vez
   - Mejora la legibilidad del código
*/