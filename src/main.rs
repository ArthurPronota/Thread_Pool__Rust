use rayon::ThreadPoolBuilder;
use rayon::prelude::* ;

fn main() {
    let pool = 
            // Создает и возвращает корректный построитель пула потоков rayon, но не инициализирует его.
            ThreadPoolBuilder::new()
            // установка количества потокков
            .num_threads(4)
            // Создает новый пул потоков, инициализированный с использованием данной конфигурации.
            .build()
            .unwrap() ;

    let res = pool
                // install позволяет выполнять код в контексте пула потоков
                .install(|| {
                    (0..1000)
                        .into_par_iter()
                        .map(|x| {
                            x * x
                        })
                        .sum::<i32>()
                }
            ) ;

    println!("Result: {res}") ;
}
