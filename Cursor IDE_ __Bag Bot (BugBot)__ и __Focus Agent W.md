<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Cursor IDE: **Bag Bot (BugBot)** и **Focus Agent Window** — Полный Обзор (Октябрь 2025)

Окей, давай сразу по делам. Cursor IDE внедрил две мощные фичи, которые серьёзно меняют подход к кодингу и ревью кода — **BugBot** и **Agent Window (Focus Agent Window)**. Это не какая-то маркетинговая хуйня, а реальные инструменты, которые экономят время и ловят баги до того, как они попадут в продакшн.[^1_1][^1_2][^1_3]

## **BugBot** — Автоматический Ревьюер Pull Request'ов на GitHub

### Что это такое?

**BugBot** — это AI-агент, встроенный в Cursor IDE, который **автоматически анализирует твои pull request'ы на GitHub** и находит баги, логические ошибки, уязвимости безопасности и нарушения стандартов кода. Работает прямо в твоём GitHub-workflow: когда ты создаёшь PR, BugBot сканирует изменения и постит комментарии прямо в интерфейсе GitHub, как если бы это делал живой ревьюер.[^1_2][^1_3][^1_4][^1_5]

### Как работает BugBot?

1. **Интеграция с GitHub**: Ты подключаешь Cursor к своему GitHub-аккаунту или организации через `cursor.com/dashboard → Integrations`.[^1_5][^1_6]
2. **Выбор репозиториев**: Выбираешь, в каких репозиториях должен работать BugBot, и активируешь его.[^1_7][^1_5]
3. **Автоматический запуск**: Когда создаётся новый PR, BugBot автоматически анализирует код (если настроено) или запускается по команде `@bugbot run` в комментариях к PR.[^1_8][^1_6][^1_5]
4. **Поиск багов**: BugBot использует те же AI-модели, что и Cursor Agent, чтобы найти:
    - **Логические ошибки** (например, закомментированный код, который ломает функциональность)[^1_9][^1_10]
    - **Проблемы безопасности** (например, незащищённые JWT-токены, ошибки обработки исключений)[^1_6]
    - **Нарушения best practices** (например, отсутствие проверок на `null`, неправильная обработка ошибок)[^1_8]
5. **Комментарии в PR**: BugBot постит детальные комментарии с описанием найденных проблем прямо в GitHub.[^1_10][^1_5]
6. **Исправление в один клик**: Если BugBot нашёл баг, ты можешь кликнуть кнопку **"Fix in Cursor"**, и Cursor откроется с уже подготовленным контекстом и промптом для исправления.[^1_2][^1_5][^1_6]

### Настройки BugBot

BugBot даёт тебе гибкий контроль над тем, как и когда он работает:[^1_5]

- **Only Run when Mentioned**: Запускается только когда ты пишешь `@bugbot run` в комментарии (экономит кредиты).
- **Only Run Once**: Запускается один раз на PR, игнорирует новые коммиты (полезно для больших PR).[^1_5]
- **Hide "No Bugs Found" Comments**: Скрывает комментарии, если BugBot ничего не нашёл (меньше спама).[^1_5]


### Пример работы

Допустим, ты создал PR, где случайно закомментировал критический код. BugBot находит это, постит комментарий в GitHub:

```
⚠️ Potential Bug: Disabled by Commented Code
The entire route handler in `route.ts` is commented out,
which will cause 404 errors for this endpoint.
```

Ты кликаешь **"Fix in Cursor"**, Cursor открывается, ты принимаешь фикс, пушишь изменения — готово.[^1_10][^1_6]

### Плюсы и минусы

**Плюсы:**

- **Ловит реальные баги**, которые люди могут пропустить (особенно в AI-generated коде).[^1_6][^1_8]
- **Экономит время на ревью**: BugBot работает как junior reviewer, освобождая senior-разработчиков.[^1_4][^1_8]
- **Интеграция с GitHub**: Всё прямо в PR, не нужно лезть в другие инструменты.[^1_5]

**Минусы:**

- **False positives**: Иногда BugBot находит "баги", которых на самом деле нет (например, путает параметры, которые не перепутаны, или предупреждает о `null`-checks там, где это не нужно).[^1_11][^1_8]
- **Платная фича**: После бесплатного триала стоит **\$40/месяц** (дороже, чем GitHub Copilot). Также есть вариант использовать существующую подписку Claude Code без доплаты.[^1_8]
- **Не всегда запускается**: Есть баги с тем, что BugBot иногда не запускается на новых репозиториях или требует ручного рефреша прав доступа.[^1_12][^1_7]


### Реальная эффективность

По отзывам разработчиков, BugBot **за месяц работы нашёл около 10 реальных багов**, включая такие тонкие проблемы, как перепутанные списки телефонов в коде. Это не революция, но полезное дополнение к ручному ревью, особенно если команда маленькая и на ревью времени мало.[^1_8]

***

## **Focus Agent Window (Agent Window)** — Управление Cursor через Cmd/Ctrl+E

### Что это такое?

**Agent Window (Focus Agent Window)** — это **отдельное окно или интерфейс в Cursor IDE**, который позволяет управлять фоновыми AI-агентами и взаимодействовать с Cursor через браузероподобный интерфейс. Это **не просто чат с AI**, а полноценная панель управления, где ты можешь:[^1_13][^1_14][^1_15]

- Запускать **фоновые агенты (Background Agents)**, которые работают параллельно с тобой.[^1_14][^1_16][^1_13]
- Управлять **задачами агентов** в реальном времени.[^1_13]
- Использовать **встроенный браузер** для тестирования веб-приложений.[^1_17][^1_9][^1_13]


### Как открыть Agent Window?

**Основной способ**: Нажми **Cmd+E (macOS)** или **Ctrl+E (Windows/Linux)**.[^1_15][^1_16][^1_14]

Это откроет **Background Agent control panel** — интерфейс для управления фоновыми агентами. В новых версиях Cursor (октябрь 2025) есть также кнопка **"Focus Agent Window"** в AI-панели, которая переключает тебя между обычным редактором и Agent Window.[^1_18][^1_14][^1_15][^1_13]

### Что такое Background Agents?

**Background Agents** — это **асинхронные AI-агенты**, которые работают в удалённой среде (обычно Ubuntu-based виртуалка) и могут **параллельно выполнять задачи** без твоего прямого участия. Они:[^1_19][^1_20]

- **Клонируют твой репозиторий** с GitHub.[^1_20]
- Работают на **отдельной ветке** (`cursor/<название-задачи>`).[^1_19]
- Выполняют код, запускают тесты, устанавливают пакеты, ищут в интернете — всё автономно.[^1_20]
- По завершении создают **pull request** с результатами работы.[^1_19][^1_20]


### Как использовать Background Agents через Agent Window?

1. **Открой Agent Window**: Нажми **Cmd/Ctrl+E**.[^1_16][^1_14]
2. **Переключись в режим "Background"**: В чат-панели выбери режим **"Background"** вместо **"Agent"** или **"Ask"**.[^1_20][^1_19]
3. **Задай задачу**: Напиши промпт, например:

```
Add authentication to this Express app using JWT tokens
```

4. **Агент начнёт работу**: Cursor клонирует репо, создаст ветку, установит зависимости, напишет код, запустит тесты.[^1_19][^1_20]
5. **Получи уведомление**: Когда агент закончит, ты получишь уведомление (можешь продолжать работать над другими задачами).[^1_19]
6. **Ревью и мердж**: Зайди в Agent Window, проверь изменения, создай PR или смёржь локально.[^1_19]

### Особенности Agent Window

- **Параллельная работа**: Ты можешь запустить несколько Background Agents одновременно, каждый на своей задаче.[^1_20]
- **Управление из любого места**: Agent Window доступен через веб-браузер (desktop, mobile, PWA), не только через Cursor IDE.[^1_21][^1_20]
- **Изолированная среда**: Агенты работают в изолированной Ubuntu-машине с интернетом, можно настроить Dockerfile и `setup.sh` для кастомной конфигурации.[^1_20]


### Пример workflow с Background Agents

Допустим, у тебя в бэклоге задача: *"Добавить rate limiting для API endpoints"*.

1. Нажми **Cmd+E**, переключись в **Background** mode.[^1_16][^1_19]
2. Напиши:

```
@auth-service.ts @database-schema.sql @security-docs
implement rate limiting for API endpoints following our existing patterns
```

3. Агент создаст план, установит нужные пакеты (например, `express-rate-limit`), напишет middleware, обновит роуты, запустит тесты.[^1_19]
4. Ты продолжаешь работать над другой задачей.[^1_19]
5. Через 10-15 минут получаешь уведомление: **"Task completed: Rate limiting implemented"**.[^1_19]
6. Открываешь Agent Window, проверяешь diff, создаёшь PR.[^1_19]

***

## **Встроенный Браузер в Agent Window** — `@Browser` Command

### Что это?

С октября 2025 года Cursor Agent может **управлять веб-браузером** напрямую из Agent Window через команду `@Browser`. Это значит, что Cursor может:[^1_22][^1_9][^1_17]

- **Открывать твоё веб-приложение** (например, `http://localhost:3000`).[^1_17]
- **Тестировать UI** (кликать кнопки, заполнять формы, проверять функциональность).[^1_9][^1_17]
- **Делать скриншоты** и анализировать визуальные проблемы.[^1_9][^1_17]
- **Дебажить JavaScript-ошибки** через консоль браузера.[^1_22][^1_9]
- **Проверять network requests** и производительность.[^1_17][^1_22]


### Как использовать `@Browser`?

1. **Открой Agent Window**: Нажми **Cmd/Ctrl+E** или открой chat-панель.[^1_13][^1_17]
2. **Введи команду `@Browser`**:

```
@Browser Test my bakery website at http://localhost:3000 for functionality, performance, and UI issues.
```

3. **Cursor откроет браузер** (обычно Chrome), зайдёт на указанный URL, проанализирует страницу, найдёт ошибки (например, 404 на каком-то роуте, кривая валидация форм, медленные запросы).[^1_23][^1_17]
4. **Получишь отчёт**: Cursor покажет список проблем с рекомендациями по фиксу.[^1_17]

### Примеры использования `@Browser`

- **Debug JavaScript errors**:

```
@Browser Debug JavaScript errors on my site at http://localhost:3000
```

- **Проверка accessibility**:

```
@Browser Audit accessibility for my homepage
```

- **Конвертация дизайна в код**:

```
@Browser Take a screenshot of my checkout page and suggest UI improvements
```

- **Автоматизация тестов**:

```
@Browser Test form validation on http://localhost:3000/checkout
```


### Реальный кейс использования

Один разработчик использовал `@Browser` для автоматизации парсинга данных из публичного реестра недвижимости: Cursor открывал сайт, кликал по записям, искал нужные поля, вытаскивал адреса и сохранял в CSV. Задача, которая обычно занимает часы ручного кликания, была сделана за 10 минут через Cursor Agent.[^1_23]

### Ограничения `@Browser`

- **Работает только с установленным Chrome**: Cursor ищет Chrome на твоей машине, если его нет — фича не работает.[^1_24][^1_25]
- **Иногда не работает**: Есть баги, когда `@Browser` не видит браузер или не может подключиться.[^1_25][^1_24]
- **Требует настройки**: Нужно убедиться, что в настройках Cursor включён **"Browser Automation"** и показано **"Ready (Chrome detected)"**.[^1_25]

***

## **Сравнение: Agent Window vs. Composer vs. Chat**

Cursor IDE теперь имеет несколько режимов работы с AI, и это **часто сбивает с толку**. Давай разберёмся:[^1_26][^1_27][^1_28]


| **Режим** | **Открытие** | **Что делает** | **Когда использовать** |
| :-- | :-- | :-- | :-- |
| **Chat (Cmd+L)** | Cmd/Ctrl+L | Отвечает на вопросы, объясняет код, предлагает варианты. **Не редактирует код напрямую**[^1_27][^1_28]. | Для вопросов типа "Как это работает?", "Какой best practice?"[^1_28] |
| **Composer (Cmd+I)** | Cmd/Ctrl+I | **Редактирует код** в нескольких файлах, показывает diff, применяет изменения одним кликом[^1_26][^1_27][^1_29]. | Для задач типа "Добавь аутентификацию", "Рефактор этого модуля"[^1_26][^1_28] |
| **Agent Mode** (в Composer) | В Composer выбери "Agent" | **Автономно** выбирает файлы, запускает команды, итеративно фиксит ошибки[^1_26][^1_27]. | Для сложных задач: "Добавь новую фичу с тестами"[^1_26][^1_28] |
| **Background Agents (Cmd+E)** | Cmd/Ctrl+E | Работает **параллельно** в фоне, создаёт отдельную ветку, делает PR[^1_19][^1_20]. | Для долгих задач, которые не нужно контролировать в реальном времени[^1_19][^1_20] |
| **Agent Window** | Cmd/Ctrl+E или кнопка "Focus Agent Window" | **Интерфейс управления** фоновыми агентами, встроенный браузер[^1_13][^1_14]. | Для мониторинга агентов, тестирования через `@Browser`[^1_13][^1_17] |

### Какой режим выбрать?

- **Chat**: Ты хочешь понять код, задать вопрос, получить совет. Изменения не применяются автоматически.[^1_28]
- **Composer (Normal)**: Ты хочешь сделать небольшой фикс в 1-2 файлах, контролируя каждый шаг.[^1_26]
- **Composer (Agent Mode)**: Ты хочешь добавить фичу, которая затрагивает много файлов, и готов доверить AI автоматический запуск команд.[^1_27][^1_26]
- **Background Agents**: Ты хочешь делегировать задачу полностью (например, "Добавь Stripe payment integration") и продолжить работать над другим.[^1_20][^1_19]

***

## **Настройка и Требования**

### Для BugBot:

1. **Подписка Cursor Pro или Business** (\$20-40/мес).[^1_8]
2. **GitHub аккаунт** с доступом к репозиториям.[^1_6][^1_5]
3. **Активация BugBot** через `cursor.com/dashboard → Integrations → GitHub`.[^1_6][^1_5]

### Для Background Agents и Agent Window:

1. **Отключи Privacy Mode** в настройках Cursor (Background Agents работают в удалённой среде, что требует отправки кода).[^1_20]
2. **Включи Usage-Based Spending**: Пополни аккаунт минимум на \$10-20, так как Background Agents используют Max models (дороже).[^1_20]
3. **Подключи GitHub**: Дай Cursor read-write доступ к репозиториям.[^1_20]
4. **Настрой окружение**: Опционально создай `Dockerfile` и `setup.sh` для кастомной конфигурации (например, специфичные версии Node, Ruby, Postgres).[^1_20]

### Для `@Browser`:

1. **Установи Google Chrome**.[^1_24][^1_25]
2. **Проверь настройки**: Зайди в Cursor Settings → Features → Browser Automation, убедись что показано **"Ready (Chrome detected)"**.[^1_25]

***

## **Заключение**

**BugBot** и **Agent Window (Focus Agent Window)** — это **не хайп, а реальные инструменты**, которые меняют workflow в Cursor IDE. BugBot ловит баги в PR до того, как они попадут в продакшн, а Agent Window с Background Agents и `@Browser` дают тебе **возможность параллелить задачи** и **автоматизировать тестирование** веб-приложений.[^1_3][^1_1][^1_2][^1_13][^1_17][^1_8][^1_19][^1_20]

Да, есть **баги и ограничения** (false positives в BugBot, проблемы с подключением браузера, необходимость платить за Max models), но если ты работаешь над большими проектами и хочешь экономить время на ревью и рутине — эти фичи стоят того, чтобы попробовать.[^1_11][^1_24][^1_25][^1_8][^1_19][^1_20]

**Главное:**

- **BugBot** = автоматический ревьюер PR на GitHub, ловит баги, создаёт комментарии, даёт кнопку "Fix in Cursor".[^1_3][^1_2][^1_5]
- **Agent Window (Cmd/Ctrl+E)** = интерфейс управления фоновыми агентами, которые работают параллельно, создают ветки и PR.[^1_14][^1_13][^1_19][^1_20]
- **`@Browser`** = Cursor управляет браузером, тестирует UI, дебажит JavaScript, делает скриншоты.[^1_22][^1_9][^1_17]

Попробуй сам, экспериментируй с настройками, и решай, нужны ли тебе эти фичи для твоего workflow. Главное — не забывай проверять, что агенты делают, потому что AI — это не магия, а **инструмент, который требует контроля**.[^1_20]
<span style="display:none">[^1_100][^1_101][^1_102][^1_103][^1_104][^1_30][^1_31][^1_32][^1_33][^1_34][^1_35][^1_36][^1_37][^1_38][^1_39][^1_40][^1_41][^1_42][^1_43][^1_44][^1_45][^1_46][^1_47][^1_48][^1_49][^1_50][^1_51][^1_52][^1_53][^1_54][^1_55][^1_56][^1_57][^1_58][^1_59][^1_60][^1_61][^1_62][^1_63][^1_64][^1_65][^1_66][^1_67][^1_68][^1_69][^1_70][^1_71][^1_72][^1_73][^1_74][^1_75][^1_76][^1_77][^1_78][^1_79][^1_80][^1_81][^1_82][^1_83][^1_84][^1_85][^1_86][^1_87][^1_88][^1_89][^1_90][^1_91][^1_92][^1_93][^1_94][^1_95][^1_96][^1_97][^1_98][^1_99]</span>

<div align="center">⁂</div>

[^1_1]: https://cursor.com

[^1_2]: https://cursor.com/bugbot

[^1_3]: https://cursor.com/docs/bugbot

[^1_4]: https://apidog.com/blog/cusor-1-0-bugbot/

[^1_5]: https://www.instructa.ai/blog/cursor-ai/cursor-bugbot

[^1_6]: https://www.youtube.com/watch?v=8USlEyGf37E

[^1_7]: https://forum.cursor.com/t/bugbot-is-not-running-on-newly-configured-repository/130916

[^1_8]: https://madewithlove.com/blog/automatic-pull-request-reviewing-with-cursors-bugbot/

[^1_9]: https://www.youtube.com/watch?v=mqBReN7yNak

[^1_10]: https://www.youtube.com/watch?v=4Jw5yJgj3pQ

[^1_11]: https://forum.cursor.com/t/bugbot-raising-invalid-bug-reports/136693

[^1_12]: https://forum.cursor.com/t/bugbot-not-running/134970

[^1_13]: https://www.youtube.com/watch?v=JCn-4Ith0bM

[^1_14]: https://cursor.com/docs/configuration/kbd

[^1_15]: https://docs.cursor.com/advanced/keyboard-shortcuts

[^1_16]: https://cursor.com/docs/background-agent

[^1_17]: https://apidog.com/blog/cursor-browser-control/

[^1_18]: https://forum.cursor.com/t/make-the-editor-window-become-the-agent-window-and-vice-versa-by-clicking-a-button-on-the-top-bar-so-you-dont-need-to-open-another-window-based-on-ryos-concept-layout/134851

[^1_19]: https://www.youtube.com/watch?v=0ctWRkOqKFc

[^1_20]: https://stevekinney.com/courses/ai-development/cursor-background-agents

[^1_21]: https://techcrunch.com/2025/06/30/cursor-launches-a-web-app-to-manage-ai-coding-agents/

[^1_22]: https://cursor.com/docs/agent/browser

[^1_23]: https://www.reddit.com/r/learnAIAgents/comments/1nvp3l9/cursor_just_dropped_browser_control_its_actually/

[^1_24]: https://forum.cursor.com/t/cursor-browser-agent-doesnt-work/137242

[^1_25]: https://forum.cursor.com/t/browser-feature-not-functioning/137383

[^1_26]: https://forum.cursor.com/t/composer-and-agent-mode/51443

[^1_27]: https://stack.convex.dev/6-tips-for-improving-your-cursor-composer-and-convex-workflow

[^1_28]: https://www.reddit.com/r/cursor/comments/1igfmcf/when_to_use_chat_vs_composer_normal_and_agent/

[^1_29]: https://www.youtube.com/watch?v=TAxyccTxxcY

[^1_30]: https://arxiv.org/pdf/2403.08299.pdf

[^1_31]: https://arxiv.org/pdf/2305.05662.pdf

[^1_32]: http://arxiv.org/pdf/2406.09577.pdf

[^1_33]: https://arxiv.org/html/2410.07002v1

[^1_34]: http://arxiv.org/pdf/2406.11317.pdf

[^1_35]: http://arxiv.org/pdf/2411.16100.pdf

[^1_36]: https://arxiv.org/pdf/2402.07456.pdf

[^1_37]: https://arxiv.org/pdf/2402.11635.pdf

[^1_38]: https://www.reddit.com/r/cursor/comments/1ltcer7/cursors_stealth_bait_and_switch_from_unlimited_to/

[^1_39]: https://www.youtube.com/watch?v=3289vhOUdKA

[^1_40]: https://skywork.ai/blog/cursor-ai-review-2025-agent-refactors-privacy/

[^1_41]: https://collabnix.com/cursor-ai-deep-dive-technical-architecture-advanced-features-best-practices-2025/

[^1_42]: https://www.youtube.com/watch?v=oWz-0E5WzC4

[^1_43]: https://forum.cursor.com/t/open-ide-cursor-by-default-in-agent-mode-beta-agent-window/132985

[^1_44]: https://skywork.ai/blog/cursor-1-7-review-2025-ai-agent-features-team-rules/

[^1_45]: https://pivot-to-ai.com/2025/04/16/cursors-ai-powered-tech-support-vibe-codes-a-customer-revolt/

[^1_46]: https://apidog.com/blog/how-to-use-cursor-agent-mode/

[^1_47]: https://www.grow-fast.co.uk/blog/cursor-ai-development-teams-shipping-3x-faster-october-2025

[^1_48]: https://cursor.com/features

[^1_49]: https://cursor.com/blog/plan-mode

[^1_50]: https://www.reddit.com/r/ChatGPTCoding/comments/1hxh5pw/is_cursor_worth_the_hype/

[^1_51]: https://forum.cursor.com/t/agent-window-tools/135523

[^1_52]: https://dev.to/heymarkkop/my-top-cursor-tips-oct-2025-3bi2

[^1_53]: https://thehackernews.com/2025/08/cursor-ai-code-editor-vulnerability.html

[^1_54]: https://arxiv.org/pdf/2209.10062.pdf

[^1_55]: http://arxiv.org/pdf/2412.03905.pdf

[^1_56]: https://arxiv.org/pdf/2406.12196.pdf

[^1_57]: http://arxiv.org/pdf/2304.00385.pdf

[^1_58]: https://arxiv.org/pdf/2402.02961.pdf

[^1_59]: http://arxiv.org/pdf/2402.14471.pdf

[^1_60]: https://forum.cursor.com/c/bug-report/6?page=3

[^1_61]: https://forum.cursor.com

[^1_62]: https://github.com/cursor/cursor/issues/3195

[^1_63]: https://forum.cursor.com/c/bug-report/6?page=5\&per_page=50

[^1_64]: https://forum.cursor.com/t/composer-hijacks-focus/131670

[^1_65]: https://forum.cursor.com/t/cannot-close-agent-changes-tab-in-agent-window/134259

[^1_66]: https://cursor.com/docs/cli/cookbook/code-review

[^1_67]: https://forum.cursor.com/c/bug-report/6?page=4\&per_page=50

[^1_68]: https://arxiv.org/pdf/2402.07939.pdf

[^1_69]: http://arxiv.org/pdf/2409.16120.pdf

[^1_70]: https://arxiv.org/html/2502.18525v1

[^1_71]: https://arxiv.org/html/2410.08164

[^1_72]: http://arxiv.org/pdf/2410.05243.pdf

[^1_73]: https://forum.cursor.com/t/why-does-the-ui-lose-focus-on-the-chat-window-mid-sentence/134234

[^1_74]: https://www.reddit.com/r/cursor/comments/1jfrbm5/bring_back_the_toggle_between_ask_and_agent_chat/

[^1_75]: https://forum.cursor.com/t/agent-window-changes-code-does-not-use-correct-font/134539

[^1_76]: https://cursor.com/docs/agent/overview

[^1_77]: https://forum.cursor.com/t/whats-the-difference-between-chat-and-composer/21879

[^1_78]: https://www.f22labs.com/blogs/cursor-agent-vs-claude-code-a-comparative-guide-in-2025/

[^1_79]: http://arxiv.org/pdf/2407.13032.pdf

[^1_80]: https://arxiv.org/pdf/2503.02950.pdf

[^1_81]: https://arxiv.org/pdf/2306.00245.pdf

[^1_82]: http://arxiv.org/pdf/2401.10935.pdf

[^1_83]: https://arxiv.org/html/2502.18356v1

[^1_84]: https://forum.cursor.com/t/cursor-agent-window-options-button-are-gone-in-1-8-0-patch-26-beta/137258

[^1_85]: https://forum.cursor.com/t/ctrl-enter-on-ai-pane-chat-runs-query-to-background-mode-no-matter-what/124445

[^1_86]: https://www.reddit.com/r/cursor/comments/1iknp8z/cursor_with_browser_use/

[^1_87]: https://www.reddit.com/r/cursor/comments/1nx6ris/recent_browser_update_in_the_agent_mode_is_fire/

[^1_88]: https://www.semanticscholar.org/paper/7ddde838439a52ee50f66c2eb0deb1f0cfed9480

[^1_89]: https://www.semanticscholar.org/paper/33eb699b7c1ae9cdffd7bdbd78d317f9a6299de1

[^1_90]: https://www.semanticscholar.org/paper/a0d7fd37affc7e455669e142f8235097f8b2ac28

[^1_91]: https://www.semanticscholar.org/paper/e444640a2b7341188a9c3185595ab5f7248ba46c

[^1_92]: https://www.semanticscholar.org/paper/1a45f4aa990d000c4ce6715900193e0f447d5b50

[^1_93]: https://arxiv.org/pdf/1009.3714.pdf

[^1_94]: http://arxiv.org/pdf/2402.09745.pdf

[^1_95]: https://arxiv.org/html/2409.10741v1

[^1_96]: https://eprints.soton.ac.uk/406516/1/demo34_ATS.pdf

[^1_97]: https://onlinelibrary.wiley.com/doi/pdfdirect/10.1002/stvr.1771

[^1_98]: http://arxiv.org/pdf/1611.07862.pdf

[^1_99]: https://ph.pollub.pl/index.php/iapgos/article/download/5374/4295

[^1_100]: https://www.youtube.com/watch?v=0rmG7rYmAkE

[^1_101]: https://www.reddit.com/r/cursor/comments/1l4t4v0/anyone_used_bugbot_for_automatic_code_review_in/

[^1_102]: https://linear.app/changelog/2025-08-21-cursor-agent

[^1_103]: https://forum.cursor.com/t/cursor-needs-awareness-of-the-ui/101532

[^1_104]: https://forum.cursor.com/t/bugbot-re-reviewing-prs-on-push-despite-being-configured-to-only-run-once/101051


---

# Оптимальная версия Cursor для BugBot и Browser Agent (Октябрь 2025)

Окей, давай сразу по делу — **какую версию Cursor поставить, чтобы иметь BugBot и Browser Agent без геморроя**. Вот конкретные рекомендации на основе актуальной инфы за октябрь 2025.[^2_1][^2_2][^2_3][^2_4]

***

## **TL;DR: Ставь Cursor 1.7.54 (Stable)**

**Версия:** Cursor **1.7.54** (последняя стабильная на 22 октября 2025)[^2_3][^2_5]

**Почему:**

- **BugBot** работает стабильно (вышел из беты в июле 2025)[^2_6][^2_7]
- **Browser Agent** (`@Browser` команда) доступен и функционален[^2_8][^2_9][^2_10]
- **Plan Mode** (планирование сложных задач) включён[^2_11][^2_8]
- **Agent Window (Cmd/Ctrl+E)** работает без критических багов[^2_4]
- **Hooks (beta)** для контроля поведения агентов[^2_12][^2_8]
- **Team Rules** для централизованных правил в командах[^2_8][^2_12]

**Где скачать:**

- **Официальный сайт:** [cursor.com/download](https://cursor.com/download)[^2_13]
- **Архив версий:** [cursorhistory.com](https://cursorhistory.com) — есть все старые версии для Windows/macOS/Linux[^2_3]

***

## **Stable vs. Nightly: Что выбрать?**

### **Cursor 1.7.x (Stable) — Рекомендуется**

**Версия:** 1.7.54 (на 22 октября 2025)[^2_5][^2_3]

**Плюсы:**

- **Все фичи работают**: BugBot, Browser Agent, Plan Mode, Background Agents, Hooks (beta)[^2_1][^2_4][^2_8]
- **Стабильность**: Протестирована, минимум критических багов[^2_2][^2_4]
- **Частые патчи**: После выхода 1.7 команда Cursor активно фиксит баги (обычно 2-3 патча в неделю)[^2_14]
- **Официальная документация**: Все фичи задокументированы в changelog[^2_1][^2_8]

**Минусы:**

- **Hooks ещё в beta**: Документация по Hooks неполная, но фича работает[^2_12][^2_4][^2_8]
- **Иногда глючит Agent Window**: Редкие баги с Browser Automation ("unable to check status") в Agent Window, но в основном editor окне работает[^2_15][^2_16][^2_17]


### **Cursor 1.8.x (Nightly/Pre-release) — НЕ рекомендуется сейчас**

**Статус:** Версия 1.8.x находится в **pre-release/nightly**, очень нестабильна (октябрь 2025)[^2_18][^2_19][^2_14]

**Проблемы:**

- **Ничего не работает**: Расширения, Git, Agent, Chat, Composer — всё сломано в первых билдах 1.8[^2_18]
- **Agent Autocomplete не работает**: Даже после включения в настройках[^2_18]
- **Потеря данных**: Некоторые пользователи теряли историю чатов и настройки после апдейта на 1.8[^2_20][^2_18]
- **Нет полноценного changelog**: Непонятно, что вообще добавили в 1.8[^2_21][^2_18]

**Вердикт:** **НЕ СТАВЬ 1.8.x**, если хочешь стабильности. Это experimental build для тех, кто хочет тестировать и сообщать о багах.[^2_19][^2_22][^2_23]

***

## **Что нужно для работы BugBot и Browser Agent?**

### **1. BugBot**

**Требования:**

- **Версия Cursor:** 1.0+ (вышел из беты в июле 2025)[^2_7][^2_6]
- **Подписка:** Cursor **Pro** (\$20/мес) или **Business** (\$40/мес)[^2_24][^2_6]
- **GitHub интеграция:** Подключить через `cursor.com/dashboard → Integrations → GitHub`[^2_25][^2_26]

**Где включить:**

1. Зайди на [cursor.com/dashboard](https://cursor.com/dashboard)
2. Вкладка **"Bugbot"**
3. Подключи GitHub, выбери репозитории
4. Настрой режим работы: "Only Run when Mentioned" или "Auto Run on New PR"[^2_25]

**Фичи доступны с 1.0:**

- Автоматический анализ PR на GitHub[^2_27][^2_28][^2_6]
- Комментарии с найденными багами[^2_29][^2_25]
- Кнопка "Fix in Cursor" для быстрого исправления[^2_26][^2_27][^2_25]
- **Team Rules для BugBot** (с версии 1.7)[^2_8][^2_12]


### **2. Browser Agent (`@Browser`)**

**Требования:**

- **Версия Cursor:** 1.7+ (Browser Control появился в 1.7, октябрь 2025)[^2_30][^2_9][^2_8]
- **Установленный Chrome:** Cursor ищет Google Chrome на машине[^2_16][^2_17]
- **Настройка:** Settings → Features → Browser Automation → должно быть **"Ready (Chrome detected)"**[^2_9][^2_16]

**Где включить:**

1. Открой Cursor Settings (Cmd/Ctrl+,)
2. **Features → Browser Automation**
3. Убедись, что показано **"Ready (Chrome detected)"**[^2_16]
4. Если не работает — **переустанови Chrome**, перезапусти Cursor[^2_17][^2_16]

**Фичи доступны с 1.7:**

- Команда `@Browser` для открытия веб-страниц[^2_10][^2_31][^2_30]
- Автоматическое тестирование UI (клики, заполнение форм, проверка функциональности)[^2_9][^2_10]
- Скриншоты и анализ визуальных проблем[^2_30][^2_10]
- Дебаг JavaScript-ошибок через консоль браузера[^2_31][^2_30]
- **Plan Mode** для Browser Agent (Shift+Tab в Agent input)[^2_11][^2_8]

***

## **Cursor 1.7.54: Полный список фич**

Вот что ты получаешь в **Cursor 1.7.54** (stable):[^2_4][^2_1][^2_8]

### **Core Features:**

1. **BugBot** — автоматический ревью PR на GitHub[^2_28][^2_6][^2_27]
2. **Browser Agent** — управление браузером через `@Browser`[^2_10][^2_30][^2_8]
3. **Plan Mode** — Cursor пишет план перед выполнением сложных задач (Shift+Tab)[^2_11][^2_8]
4. **Background Agents** — асинхронные агенты, работают в фоне, создают отдельные ветки[^2_32][^2_33][^2_34]
5. **Agent Window (Cmd/Ctrl+E)** — панель управления фоновыми агентами[^2_35][^2_36][^2_4]

### **New in 1.7:**

6. **Agent Autocomplete** — автозаполнение промптов на основе контекста[^2_12][^2_4][^2_8]
7. **Hooks (beta)** — кастомные скрипты для контроля поведения Agent[^2_4][^2_8][^2_12]
8. **Team Rules** — централизованные правила для команд, включая BugBot Rules[^2_8][^2_12][^2_4]
9. **Image files support** — Agent читает изображения из workspace напрямую[^2_1][^2_8]
10. **Deeplinks (beta)** — шарить промпты через ссылки[^2_4][^2_8]
11. **Menubar Agent monitor** — виджет в системном трее для мониторинга агентов[^2_8][^2_4]
12. **Sandboxed terminals** — команды выполняются в изолированной среде (безопасность)[^2_8]

### **BugBot Features (1.7+):**

13. **PR summaries** — BugBot генерирует саммари PR при создании[^2_1][^2_8]
14. **Team Rules для BugBot** — централизованные правила ревью[^2_12][^2_8]

***

## **Как правильно установить Cursor 1.7.54?**

### **Вариант 1: Официальный сайт (Рекомендуется)**

1. Зайди на [cursor.com/download](https://cursor.com/download)[^2_13]
2. Скачай **1.7.54** для твоей OS (Windows/macOS/Linux)[^2_5][^2_3]
3. **Удали старую версию** перед установкой (если есть)[^2_19]
4. Установи новую версию
5. **Первый запуск:** Курсор импортирует настройки из VS Code, если есть[^2_37]

### **Вариант 2: Архив версий (Если нужна конкретная версия)**

1. Зайди на [cursorhistory.com](https://cursorhistory.com)[^2_3]
2. Найди **Cursor 1.7.54** (22 октября 2025)[^2_3]
3. Скачай для твоей платформы:
    - **Windows:** x64 или ARM64[^2_3]
    - **macOS:** Universal, Intel, или Apple Silicon[^2_3]
    - **Linux:** x64 или ARM[^2_3]
4. Установи

### **Вариант 3: Downgrade с 1.8.x на 1.7.54**

Если случайно обновился на **1.8.x** и всё сломалось:[^2_18]

1. **Удали Cursor полностью** (включая настройки, если нужен чистый старт)[^2_19]
2. Скачай **1.7.54** с [cursorhistory.com](https://cursorhistory.com)[^2_3]
3. Установи
4. **Отключи автоапдейт:** Settings → Updates → **снять галку "Check for updates"**[^2_23]

***

## **Настройка после установки 1.7.54**

### **1. Включи Browser Agent**

1. **Settings → Features → Browser Automation**[^2_16]
2. Убедись: **"Ready (Chrome detected)"**[^2_16]
3. Если **"Unable to check status"**:
    - Установи/переустанови **Google Chrome**[^2_17][^2_16]
    - Перезапусти Cursor[^2_17]

### **2. Подключи BugBot**

1. Зайди на [cursor.com/dashboard](https://cursor.com/dashboard)[^2_25]
2. **Integrations → GitHub → Connect**[^2_26][^2_25]
3. Выбери репозитории для BugBot[^2_25]
4. Настрой **BugBot Rules** (опционально): создай `BUGBOT.md` в корне репо[^2_6]

### **3. Настрой Agent Window**

1. Нажми **Cmd/Ctrl+E** — откроется Agent Window[^2_36][^2_35][^2_4]
2. Проверь **Tools** внизу:
    - **Browser Automation** — должно быть "Ready"[^2_15][^2_16]
    - **MCP Servers** — если используешь (опционально)[^2_15]
3. Переключайся между **Background** и **Agent** режимами через селектор[^2_33][^2_34]

### **4. Включи Beta Features (опционально)**

1. **Settings → Beta**[^2_14][^2_18]
2. Включи:
    - **Agent Autocomplete** (автозаполнение промптов)[^2_4][^2_8]
    - **Hooks** (контроль Agent поведения)[^2_4][^2_8]
    - **Deeplinks** (шаринг промптов)[^2_8][^2_4]

***

## **Частые баги в 1.7.54 и как их фиксить**

### **1. Browser Automation показывает "Unable to check status"**

**Причина:** Chrome не найден или Cursor не может подключиться[^2_17][^2_16]

**Фикс:**

1. Установи **Google Chrome** (не Chromium, не Brave, именно Chrome)[^2_16][^2_17]
2. Перезапусти Cursor[^2_17]
3. Если не помогло: **Settings → Features → Browser Automation → Refresh**[^2_16]

### **2. BugBot не запускается на новых репозиториях**

**Причина:** Нужно обновить права доступа GitHub[^2_38][^2_39]

**Фикс:**

1. Зайди на [cursor.com/dashboard → Integrations → GitHub](https://cursor.com/dashboard)[^2_38]
2. **Disconnect** и **Reconnect** GitHub[^2_38]
3. Дай полный read-write доступ[^2_39][^2_38]

### **3. Agent Window теряет Tools (Browser/MCP не загружаются)**

**Причина:** Баг в Agent Window, но работает в основном editor окне[^2_15]

**Фикс:**

1. Вместо **"Open in Agent Window"** используй **"Focus Editor Window"**[^2_15]
2. Или просто работай в обычном editor, нажимай **Cmd/Ctrl+L** для Chat[^2_15]

### **4. Потеря истории чатов после апдейта**

**Причина:** Баг миграции настроек в 1.7[^2_20]

**Фикс (превентивный):**

1. **Backup** перед апдейтом:
    - Windows: `%APPDATA%\Cursor\User\`[^2_20]
    - macOS: `~/Library/Application Support/Cursor/User/`[^2_20]
    - Linux: `~/.config/Cursor/User/`[^2_20]
2. После апдейта — восстанови из backup[^2_20]

### **5. "Connection failed" ошибки во время работы Agent**

**Причина:** Проблемы с API-лимитами или сетью[^2_40]

**Фикс:**

1. Проверь подписку: хватает ли **Fast Premium Requests**[^2_41][^2_40]
2. Если используешь **VPN** — попробуй отключить[^2_40]
3. Переключись на **другую модель** (вместо Claude Sonnet — GPT-4)[^2_40]

***

## **Альтернативы, если 1.7.54 не подходит**

### **1. Cursor 1.6.45 (Stable, предыдущая версия)**

**Если нужна максимальная стабильность без новых фич:**

**Версия:** Cursor **1.6.45** (сентябрь 2025)[^2_42][^2_2]

**Плюсы:**

- **BugBot работает** (вышел в 1.0)[^2_6]
- **Background Agents работают** (вышли в 0.50)[^2_32]
- **Нет Browser Agent**, но есть **Plan Mode** (вышел в 1.6)[^2_42][^2_32]
- **Меньше багов**, чем в 1.7 (потому что старше)[^2_2][^2_14]

**Минусы:**

- **Нет Browser Agent** (только в 1.7+)[^2_8]
- **Нет Hooks** (только в 1.7+)[^2_8]
- **Нет Team Rules** (только в 1.7+)[^2_8]

**Где скачать:** [cursorhistory.com](https://cursorhistory.com) → найди версию 1.6.45[^2_3]

### **2. Cursor 1.0.x (Первая стабильная версия BugBot)**

**Если нужен только BugBot без всяких Agent фич:**

**Версия:** Cursor **1.0.x** (июнь 2025)[^2_32][^2_6]

**Плюсы:**

- **BugBot работает** (впервые вышел в 1.0)[^2_32][^2_6]
- **Максимальная стабильность** (old and tested)[^2_32]

**Минусы:**

- **Нет Browser Agent** (только в 1.7+)[^2_8]
- **Нет Background Agents** (появились в 0.50)[^2_32]
- **Нет Plan Mode** (появился позже)[^2_32]

**Где скачать:** [cursorhistory.com](https://cursorhistory.com)[^2_3]

***

## **FAQ: Быстрые ответы**

### **Q: Какую версию ставить для работы с BugBot и Browser Agent?**

**A:** Cursor **1.7.54** (stable, октябрь 2025)[^2_4][^2_3][^2_8]

### **Q: Можно ли использовать Nightly (1.8.x)?**

**A:** **НЕТ**. Версия 1.8.x сломана, ничего не работает (октябрь 2025). Жди стабильного релиза.[^2_19][^2_18]

### **Q: Browser Agent не работает — что делать?**

**A:** Установи **Google Chrome**, перезапусти Cursor, проверь **Settings → Features → Browser Automation → "Ready (Chrome detected)"**.[^2_17][^2_16]

### **Q: BugBot не запускается на новых PR — что делать?**

**A:** Пере подключи GitHub: [cursor.com/dashboard → Integrations → GitHub → Disconnect/Reconnect](https://cursor.com/dashboard).[^2_39][^2_38]

### **Q: Cursor 1.7.54 vs 1.6.45 — какая лучше?**

**A:** **1.7.54**, если нужны Browser Agent, Hooks, Team Rules. **1.6.45**, если нужна максимальная стабильность без новых фич.[^2_2][^2_14][^2_4][^2_8]

### **Q: Можно ли downgrade с 1.8 на 1.7?**

**A:** **ДА**. Удали 1.8, скачай 1.7.54 с [cursorhistory.com](https://cursorhistory.com), установи, отключи автоапдейт.[^2_23][^2_19][^2_3]

### **Q: Нужна ли подписка для BugBot и Browser Agent?**

**A:** **BugBot** — нужна **Pro** (\$20/мес) или **Business** (\$40/мес). **Browser Agent** — работает на любой подписке, включая Free (но с лимитами на Fast requests).[^2_41][^2_24][^2_6][^2_40]

### **Q: Где найти официальный changelog для 1.7.54?**

**A:** [cursor.com/changelog/1-7](https://cursor.com/changelog/1-7)[^2_1][^2_8]

***

## **Итог: Что ставить?**

| **Сценарий** | **Версия** | **Почему** |
| :-- | :-- | :-- |
| **Хочу BugBot + Browser Agent + стабильность** | **Cursor 1.7.54** | Все фичи работают, активно фиксятся баги[^2_3][^2_8][^2_4] |
| **Хочу максимальную стабильность без Browser Agent** | **Cursor 1.6.45** | Старше, меньше багов, но нет новых фич[^2_2][^2_14] |
| **Хочу только BugBot без всяких агентов** | **Cursor 1.0.x** | Первая версия с BugBot, самая стабильная[^2_32][^2_6] |
| **Хочу тестировать cutting-edge фичи** | **Cursor 1.8.x Nightly** | **НЕ рекомендуется**, всё сломано (октябрь 2025)[^2_18][^2_19] |

**Рекомендация:** Ставь **Cursor 1.7.54** и отключи автоапдейт. Это последняя стабильная версия с BugBot, Browser Agent, Plan Mode, Background Agents, и всеми новыми фичами 1.7. Подожди, пока 1.8 выйдет в stable (обычно через месяц-два после начала nightly).[^2_34][^2_33][^2_30][^2_23][^2_10][^2_11][^2_6][^2_19][^2_4][^2_3][^2_8]

**Где скачать:**

- **Официально:** [cursor.com/download](https://cursor.com/download)[^2_13]
- **Архив версий:** [cursorhistory.com](https://cursorhistory.com)[^2_3]

Всё, ставь 1.7.54 и работай.[^2_4][^2_3][^2_8]
<span style="display:none">[^2_43][^2_44][^2_45][^2_46][^2_47][^2_48][^2_49][^2_50][^2_51][^2_52][^2_53][^2_54][^2_55][^2_56][^2_57][^2_58][^2_59][^2_60][^2_61][^2_62][^2_63][^2_64]</span>

<div align="center">⁂</div>

[^2_1]: https://cursor.com/changelog

[^2_2]: https://skywork.ai/blog/cursor-1-7-vs-1-6-2025-comparison-upgrade-guide/

[^2_3]: https://cursorhistory.com

[^2_4]: https://forum.cursor.com/t/cursor-1-7-is-here/135333

[^2_5]: https://cursor.en.uptodown.com/windows

[^2_6]: https://cursor.com/blog/bugbot-out-of-beta

[^2_7]: http://faun.dev/c/links/faun/cursor-bugbot-is-out-of-beta/

[^2_8]: https://cursor.com/changelog/1-7

[^2_9]: https://www.youtube.com/watch?v=46VnvLFSFCU

[^2_10]: https://apidog.com/blog/cursor-browser-control/

[^2_11]: https://cursor.com/blog/plan-mode

[^2_12]: https://skywork.ai/blog/cursor-1-7-2025-agent-autocomplete-hooks-team-rules/

[^2_13]: https://cursor.com/download

[^2_14]: https://forum.cursor.com/t/discussion-new-features-in-cursor-1-7/134864?page=3

[^2_15]: https://forum.cursor.com/t/agent-window-tools/135523

[^2_16]: https://forum.cursor.com/t/browser-feature-not-functioning/137383

[^2_17]: https://forum.cursor.com/t/cursor-browser-agent-doesnt-work/137242

[^2_18]: https://forum.cursor.com/t/1-8-nothing-works/134881

[^2_19]: https://forum.cursor.com/t/nightly-mode-for-cursor/29459

[^2_20]: https://forum.cursor.com/t/cursor-reset-settings-and-chats-twice-in-two-days/138017

[^2_21]: https://forum.cursor.com/t/cursor-v2-released/138684

[^2_22]: https://www.reddit.com/r/RetroArch/comments/e4tnrc/stable_vs_nightly_what_is_intended_for_regular/

[^2_23]: https://forum.cursor.com/t/a-proposal-to-satisfy-both-early-adopters-and-stability-focused-users-in-update-delivery/44958

[^2_24]: https://madewithlove.com/blog/automatic-pull-request-reviewing-with-cursors-bugbot/

[^2_25]: https://www.instructa.ai/blog/cursor-ai/cursor-bugbot

[^2_26]: https://www.youtube.com/watch?v=8USlEyGf37E

[^2_27]: https://cursor.com/bugbot

[^2_28]: https://cursor.com/docs/bugbot

[^2_29]: https://www.youtube.com/watch?v=4Jw5yJgj3pQ

[^2_30]: https://www.youtube.com/watch?v=mqBReN7yNak

[^2_31]: https://cursor.com/docs/agent/browser

[^2_32]: https://blog.promptlayer.com/cursor-changelog-whats-coming-next-in-2026/

[^2_33]: https://www.youtube.com/watch?v=0ctWRkOqKFc

[^2_34]: https://stevekinney.com/courses/ai-development/cursor-background-agents

[^2_35]: https://www.youtube.com/watch?v=JCn-4Ith0bM

[^2_36]: https://cursor.com/docs/configuration/kbd

[^2_37]: https://blog.enginelabs.ai/cursor-ai-an-in-depth-review

[^2_38]: https://forum.cursor.com/t/bugbot-is-not-running-on-newly-configured-repository/130916

[^2_39]: https://forum.cursor.com/t/bugbot-not-running/134970

[^2_40]: https://forum.cursor.com/t/frequent-connection-failed-errors-despite-stable-internet-connection/138766

[^2_41]: https://forum.cursor.com/t/which-cursor-version-is-currently-stable-and-safe-to-use/122905

[^2_42]: https://forum.cursor.com/t/cursor-v1-6-pre-release-discussion/133043

[^2_43]: https://www.semanticscholar.org/paper/2fddb218efae6fbfe4b07d5169afe620c941ae29

[^2_44]: https://www.semanticscholar.org/paper/d5f83defcc891e007d9050448d2b1c24d18ef8a0

[^2_45]: http://ieeexplore.ieee.org/document/7302649/

[^2_46]: https://arxiv.org/html/2410.07002v1

[^2_47]: https://arxiv.org/pdf/2410.11096.pdf

[^2_48]: http://arxiv.org/pdf/2406.09577.pdf

[^2_49]: https://arxiv.org/pdf/2305.06161.pdf

[^2_50]: https://dl.acm.org/doi/pdf/10.1145/3597503.3639187

[^2_51]: https://arxiv.org/pdf/2405.08704.pdf

[^2_52]: https://arxiv.org/html/2412.08063v1

[^2_53]: https://arxiv.org/html/2403.03433v1

[^2_54]: https://cursor.com

[^2_55]: https://www.youtube.com/watch?v=3tYBbH_nFcE

[^2_56]: https://dev.to/eva_clari_289d85ecc68da48/why-every-developer-should-try-cursor-ai-in-2025-4pem

[^2_57]: https://www.reddit.com/r/cursor/comments/1nx6ris/recent_browser_update_in_the_agent_mode_is_fire/

[^2_58]: https://cursor.com/features

[^2_59]: https://www.reddit.com/r/GithubCopilot/comments/1jnboan/github_copilot_vs_cursor_in_2025_why_im_paying/

[^2_60]: https://arxiv.org/pdf/2411.07211.pdf

[^2_61]: https://arxiv.org/html/2503.06680v1

[^2_62]: http://arxiv.org/pdf/2407.08708.pdf

[^2_63]: https://arxiv.org/pdf/2409.12186.pdf

[^2_64]: https://skywork.ai/blog/cursor-1-7-review-2025-ai-agent-features-team-rules/

