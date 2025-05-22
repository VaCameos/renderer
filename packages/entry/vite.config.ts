import {defineConfig} from 'vite'
import wasm from 'vite-plugin-wasm';

export default defineConfig({
    plugins:[
        wasm()
    ],
    build: {
        // 产物输出目录，默认值就是 dist。我们使用默认值，注释掉此字段。
        // outDir: 'dist',
    
        // 参考：https://cn.vitejs.dev/config/build-options.html#build-lib
        lib: {
          // 构建的入口文件
          entry: './src/index.ts',
    
          // 产物的生成格式，默认为 ['es', 'umd']。我们使用默认值，注释掉此字段。
          // formats: ['es', 'umd'],
    
          // 当产物为 umd、iife 格式时，该模块暴露的全局变量名称
          name: 'renderer',
          // 产物文件名称
          fileName: 'index',
        },
        rollupOptions:{
            external:'@renderer/core',
            output:[
                {
                    format:'es',
                    entryFileNames:'[name].es.js',
                    
                },
                {

                }
            ]
        },
        // 为了方便学习，查看构建产物，将此置为 false，不要混淆产物代码
        minify: false,
      }
})