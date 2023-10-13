INSERT INTO publisher VALUES (
    '42387204-D02A-4381-A378-D811012EED44',
    'Penguin Random House'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO book VALUES (
    'DDD32A71-8AB9-4E1C-A558-E2E01C3F9AD7',
    'Mindset: The New Psychology of Success',
    '2007-12-25',
    '42387204-D02A-4381-A378-D811012EED44',
    E'After decades of research, world-renowned Stanford University psychologist Carol S. Dweck, Ph.D., discovered a simple but groundbreaking idea: the power of mindset. In this brilliant book, she shows how success in school, work, sports, the arts, and almost every area of human endeavor can be dramatically influenced by how we think about our talents and abilities. People with a fixed mindset—those who believe that abilities are fixed—are less likely to flourish than those with a growth mindset—those who believe that abilities can be developed. Mindset reveals how great parents, teachers, managers, and athletes can put this idea to use to foster outstanding accomplishment.\nIn this edition, Dweck offers new insights into her now famous and broadly embraced concept. She introduces a phenomenon she calls false growth mindset and guides people toward adopting a deeper, truer growth mindset. She also expands the mindset concept beyond the individual, applying it to the cultures of groups and organizations. With the right mindset, you can motivate those you lead, teach, and love—to transform their lives and your own.'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO author VALUES (
    '611CA090-A747-4BE3-82C7-A2BDD8215F96',
    'Carol S. Dweck'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO book_author VALUES (
    'DDD32A71-8AB9-4E1C-A558-E2E01C3F9AD7',
    '611CA090-A747-4BE3-82C7-A2BDD8215F96'
) ON CONFLICT (book_id, author_id) DO NOTHING;
