// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next';

import fs from 'fs';
import path from 'path';

import { stringify } from 'csv-stringify';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
	if (req.method !== 'POST') {
		res.status(405).json({ error: 'Method not allowed' });
		return;
	}
	const filePath = path.resolve(process.cwd(), 'results/extracted_drugs.csv');

	stringify(
		req.body,
		{
			header: true,
		},
		function (err, output) {
			if (err) {
				res.status(500).json({ error: err });
			}
			fs.writeFileSync(filePath, output);
		}
	);
	res.status(201).json({ success: true, message: 'File written' });
}
